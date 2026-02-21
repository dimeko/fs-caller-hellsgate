use std::path::Path;
use std::{os::raw::c_void, path::PathBuf};
use std::arch::asm;

use winapi::ctypes::c_longlong;
use windows::Win32::System::IO::{IO_STATUS_BLOCK, IO_STATUS_BLOCK_0};
use windows::Wdk::Foundation::OBJECT_ATTRIBUTES;
use windows::Win32::Foundation::{HANDLE, NTSTATUS, UNICODE_STRING};
use windows::Win32::System::Diagnostics::Debug::{IMAGE_NT_HEADERS64};
use windows::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_EXPORT_DIRECTORY, IMAGE_NT_SIGNATURE};
use windows::Win32::System::Threading::PEB_LDR_DATA;
use windows::Win32::System::WindowsProgramming::{LDR_DATA_TABLE_ENTRY};
use windows::core::PWSTR;

pub mod utils;
pub mod defs;

#[macro_export]
macro_rules! hide {
    ($e:expr) => {{
        const fn call_djb2(_str: &str) -> u64 {
            const fn call_djb2_rec(input: &[u8], mut _dw_hash: u64, _idx: usize) -> u64 {
                let mut _idx = 0;
                loop {
                    _dw_hash = _dw_hash.wrapping_shl(5).wrapping_add(_dw_hash).wrapping_add(input[_idx] as u64);
                    _idx = _idx + 1;
                    if _idx == input.len() {
                        break;
                    }
                }
                return _dw_hash
            }
            return call_djb2_rec(_str.as_bytes(), defs::DW_HASH, 0);
        }
        call_djb2($e)
    }};
}

pub struct HFile {
    handle: HANDLE,
    ntdll_dll_base: *mut c_void,
    kernel_dll_base: *mut c_void,
    offset: c_longlong,
    file_path: PathBuf,
    mode: defs::hFILE_ACCESS::HFileAccessRights
}

impl HFile {
    fn __peb_traverse(module_name: u64)  -> Option<*mut c_void> {
        unsafe {
            let mut _peb: usize;
            let mut _ldr: usize;    
            asm!(
                "mov {_peb}, gs:[0x60]",
                "mov {_ldr}, [{_peb} + 0x18]",
                _peb = out(reg) _peb,
                _ldr = out(reg) _ldr,
            );
            let peb_ldr_struct = std::ptr::read(_ldr as *const PEB_LDR_DATA);
            
            let mut current_entry = peb_ldr_struct.InMemoryOrderModuleList.Flink.sub(0x1) as usize;
            let head = current_entry;
            loop {
                
                let ldr_data_struct =  std::ptr::read(current_entry as *const LDR_DATA_TABLE_ENTRY);
                current_entry = ldr_data_struct.InMemoryOrderLinks.Flink.sub(0x1) as usize;
                            
                if current_entry == head {
                    break;
                }

                let _module_name =  match ldr_data_struct.FullDllName.Buffer.to_string() {
                    Ok(_str) => {
                        _str
                    },
                    Err(_) => continue
                };
                let module_path = Path::new(&_module_name);
                let module_filename = module_path.file_name().unwrap();

                if utils::djb2(module_filename.to_str().unwrap()) == module_name {
                    return Some(ldr_data_struct.DllBase);
                }
            }
        }
        return None;
    }

    fn __find_null_terminator(starting_add: *const u8) -> usize {
        let mut _c: isize = 0;
        unsafe {
            loop  {
                if *(starting_add.offset(_c)) == 0{
                    break;
                }
                _c = _c + 1;
            }
        }
        _c as usize
    }

    fn __find_function_address(module_base: *mut c_void, func_djb2: u64) ->  Option<*const c_void> {
        unsafe {
            let dos_header = std::ptr::read(module_base as *const IMAGE_DOS_HEADER);
            if  dos_header.e_magic != IMAGE_DOS_SIGNATURE {
                return None;
            } 
            
            let nt_headers: IMAGE_NT_HEADERS64 = std::ptr::read(
                module_base.offset(dos_header.e_lfanew as isize) as *const IMAGE_NT_HEADERS64);
            if nt_headers.Signature != IMAGE_NT_SIGNATURE {
                return None;
            }

            let export_directory: IMAGE_EXPORT_DIRECTORY = std::ptr::read(
                module_base.add(nt_headers.OptionalHeader.DataDirectory[0].VirtualAddress as usize) as *const IMAGE_EXPORT_DIRECTORY);

            let functions = module_base.add(export_directory.AddressOfFunctions as usize) as *const u32;
            let names = module_base.add(export_directory.AddressOfNames as usize) as *const u32;
            let ordinals = module_base.add(export_directory.AddressOfNameOrdinals as usize) as *const u16;

            for i in 0.. export_directory.NumberOfNames {
                let func_name_addr = module_base.add(*names.offset(i as isize) as usize) as *const u8;
                let func_name_bytes = std::slice::from_raw_parts(func_name_addr, HFile::__find_null_terminator(func_name_addr));
                let func_name_from_bytes = str::from_utf8(func_name_bytes).unwrap();
                
                if utils::djb2(func_name_from_bytes) == func_djb2 {
                    let ordinal = *ordinals.offset(i.try_into().unwrap()) as usize;
                    let fn_addr = module_base.add(
                        *functions.offset(ordinal as isize)  as usize) as *const c_void;
                        
                    return Some(fn_addr);
                }
            }
        }
        None
    }

    fn __find_nt_function_address(&self, func_djb2: u64) -> Option<*const c_void> {
        HFile::__find_function_address(self.ntdll_dll_base, func_djb2)
    }

    fn __find_kernel32_function_address(&self, func_djb2: u64) -> Option<*const c_void> {
        HFile::__find_function_address(self.kernel_dll_base, func_djb2)
    }

    fn __find_nt_function_ssn(&self, func_djb2: u64) -> Option<(u32, u64)> {
        let func_addr = match self.__find_nt_function_address(func_djb2) {
            Some(fa) => fa,
            None => {
                return None;
            }
        };

        unsafe  {
            return Some((
                *((func_addr as *const u8).add(4) as *const u32),
                (func_addr as *const u8).add(18) as u64
            ));
        }
    }

    fn __init_dll_bases_functions(&mut self) {
        self.ntdll_dll_base = match HFile::__peb_traverse(hide!("ntdll.dll")) {
            Some(_d) => _d,
            None => {
                panic!("Could not find dll base");
            }
        };

        self.kernel_dll_base = match HFile::__peb_traverse(hide!("KERNEL32.DLL")) {
            Some(_d) => _d,
            None => {
                panic!("Could not find dll base");
            }
        };
    }

    pub fn new(file_path: PathBuf, mode: defs::hFILE_ACCESS::HFileAccessRights) -> Self {
        let mut _self: HFile = Self {
            handle: HANDLE::default(),
            offset: 3,
            file_path,
            mode,
            ntdll_dll_base: std::ptr::null_mut(),
            kernel_dll_base: std::ptr::null_mut(),
        };
        _self.__init_dll_bases_functions();
        _self
    }

    pub fn get_handle(&self) -> HANDLE {
        self.handle
    }

    fn create_io_stats_block() -> IO_STATUS_BLOCK {
        IO_STATUS_BLOCK {
            Anonymous:  IO_STATUS_BLOCK_0 {
                Status: NTSTATUS(0),
            },
            Information: 0,
        }
    }

    fn create_object_attrs(file_path_unicode: *mut UNICODE_STRING) -> OBJECT_ATTRIBUTES {
        OBJECT_ATTRIBUTES {
            Length: std::mem::size_of::<OBJECT_ATTRIBUTES>() as u32,
            RootDirectory: HANDLE(0),
            ObjectName: file_path_unicode,
            Attributes: defs::hOBJECT_ATTRIBUTES::OBJ_CASE_INSENSITIVE,
            SecurityDescriptor: std::ptr::null(),
            SecurityQualityOfService: std::ptr::null(),
        }
    }

    pub fn create(file_path: PathBuf, mode: defs::hFILE_ACCESS::HFileAccessRights) -> Result<(Self, NTSTATUS), String> {
        let mut hfile = HFile::new(file_path, mode);
        let func_name = hide!("NtCreateFile");
        let (ssn, syscall_addr) = match hfile.__find_nt_function_ssn(func_name) {
            Some(_ssn) => _ssn,
            None => {
                return Err(format!("could not find ssn for {:?}", func_name));
            }
        };
        let file_path_str = hfile.file_path.to_str().unwrap();
        let w_string: widestring::U16CString = widestring::WideCString::from_str(&file_path_str).unwrap();
        let mut unicode_string = UNICODE_STRING {
            Length: (file_path_str.len() * 2) as u16,
            MaximumLength: (file_path_str.len() * 2) as u16 + 2,
            Buffer: PWSTR::from_raw(w_string.as_ptr() as *mut u16).to_owned(),
        };

        unsafe {
            defs::hNtCreateFileSsn = ssn;
            defs::hNtCreateFileSyscallAddr = syscall_addr;
        }
        let mut iosb: IO_STATUS_BLOCK = HFile::create_io_stats_block().to_owned();
        let mut obj_attrs = HFile::create_object_attrs(&mut unicode_string);
        unsafe {
            let ntstatus = defs::hNtCreateFile(
                &mut hfile.handle as *mut HANDLE,
                hfile.mode,
                &mut obj_attrs,
                &mut iosb,
                std::ptr::null_mut(),
                defs::hFILE_ATTRIBUTES::FILE_ATTRIBUTE_NORMAL,
                defs::hSHARE_ACCESS::FILE_SHARE_WRITE,
                defs::hCREATE_DISPOSITION::FILE_OVERWRITE_IF,
                defs::hCREATE_OPTIONS::FILE_SYNCHRONOUS_IO_NONALERT,
                std::ptr::null_mut(),
                0,
            );
            Ok((hfile, ntstatus))
        }
        
    }

    pub fn write(&mut self, mut buffer: Vec<u8>) -> Result<(NTSTATUS, usize), String> {
        let func_name = hide!("NtWriteFile");
        let (ssn, syscall_addr) = match self.__find_nt_function_ssn(func_name) {
            Some(_ssn) => _ssn,
            None => {
                return Err(format!("could not find ssn for {:?}", func_name));
            }
        };
        unsafe {
            defs::hNtWriteFileSyscallAddr = syscall_addr;    
            defs::hNtWriteFileSsn = ssn;
        }

        let mut iosb = HFile::create_io_stats_block().to_owned();      
        unsafe {
            let __status = defs::hNtWriteFile(
                self.handle.to_owned(), 
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut iosb,
                buffer.as_mut_ptr() as *const u8,
                buffer.len(),
                &mut self.offset as *mut c_longlong,
                0
            );
            Ok((__status, iosb.Information))
        }
    }

    pub fn open(file_path: PathBuf, mode: defs::hFILE_ACCESS::HFileAccessRights) -> Result<(Self, NTSTATUS), String> {
        let mut hfile = HFile::new(file_path, mode);
        let func_name = hide!("NtOpenFile");
        let (ssn, syscall_addr) = match hfile.__find_nt_function_ssn(func_name) {
            Some(_ssn) => _ssn,
            None => {
                return Err(format!("could not find ssn for {:?}", func_name));
            }
        };
        let file_path_str = hfile.file_path.to_str().unwrap();
        
        let mut iosb =  HFile::create_io_stats_block();
        let w_string: widestring::U16CString = widestring::WideCString::from_str(&file_path_str).unwrap();
        let mut unicode_string = UNICODE_STRING {
            Length: (file_path_str.len() * 2) as u16,
            MaximumLength: (file_path_str.len() * 2) as u16 + 2,
            Buffer: PWSTR::from_raw(w_string.as_ptr() as *mut u16).to_owned(),
        };
        unsafe {
            defs::hNtOpenFileSsn = ssn;
            defs::hNtOpenFileSyscallAddr = syscall_addr;
        }
        let mut obj_attrs = HFile::create_object_attrs(&mut unicode_string); 

        unsafe {
            let ntstatus =  defs::hNtOpenFile(
                &mut hfile.handle as *mut HANDLE,
                hfile.mode,
                &mut obj_attrs,
                &mut iosb,
                defs::hSHARE_ACCESS::FILE_SHARE_READ,
                defs::hCREATE_OPTIONS::FILE_NON_DIRECTORY_FILE
            );
            Ok((hfile, ntstatus))
        }
    }

    pub fn close(&mut self) -> Result<bool, String> {
        let func_name = hide!("CloseHandle");
        let func_addr = match self.__find_kernel32_function_address(func_name) {
            Some(fa) => fa,
            None => {
                return Err(format!("Could not find close function"));
            }
        };
        unsafe {
            let func: fn(HANDLE) -> bool =
                std::mem::transmute(func_addr);

            let result = func(self.handle);
            println!("Result: {}", result);
            Ok(result)
        }
    }

    pub fn delete(&self) -> Result<NTSTATUS, String> {
        let func_name = hide!("NtDeleteFile");
        let (ssn, syscall_addr) = match self.__find_nt_function_ssn(func_name) {
            Some(_ssn) => _ssn,
            None => {
                return Err(format!("could not find ssn for {:?}", func_name));
            }
        };

        let file_path_str = self.file_path.to_str().unwrap();
        let w_string: widestring::U16CString = widestring::WideCString::from_str(&file_path_str).unwrap();
        let mut unicode_string = UNICODE_STRING {
            Length: (file_path_str.len() * 2) as u16,
            MaximumLength: (file_path_str.len() * 2) as u16 + 2,
            Buffer: PWSTR::from_raw(w_string.as_ptr() as *mut u16).to_owned(),
        };
        unsafe {
            defs::hNtDeleteFileSsn = ssn;
            defs::hNtDeleteFileSyscallAddr = syscall_addr;
        }
        let mut obj_attrs = HFile::create_object_attrs(&mut unicode_string); 
        unsafe {
            let ntstatus = defs::hNtDeleteFile(&mut obj_attrs);
            Ok(ntstatus)
        }
    }

    pub fn read(&mut self, n_bytes: usize) -> Result<(NTSTATUS, Vec<u8>), String> {
        let func_name = hide!("NtReadFile");
        let (ssn, syscall_addr) = match self.__find_nt_function_ssn(func_name) {
            Some(_ssn) => _ssn,
            None => {
                return Err(format!("could not find ssn for {:?}", func_name));
            }
        };
        unsafe {
            defs::hNtReadFileSsn = ssn;
            defs::hNtReadFileSyscallAddr = syscall_addr;
        }
        let mut iosb = HFile::create_io_stats_block();

        let mut buffer = Vec::with_capacity(n_bytes);

        unsafe {
            let ntstatus = defs::hNtReadFile(
                self.handle,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut iosb,
                buffer.as_mut_ptr(),
                n_bytes as u32,
                &self.offset as *const c_longlong,
                std::ptr::null_mut());
            if ntstatus.is_ok() {
                self.offset = self.offset + n_bytes as i64;
                buffer.set_len(iosb.Information as usize);
                // or buffer.truncate(iosb.Information as usize);
            }
            Ok((ntstatus, buffer))
        }
    }

    pub fn info(&mut self) -> Result<NTSTATUS, String> {
        let func_name = hide!("NtQueryInformationFile");
        let (ssn, syscall_addr) = match self.__find_nt_function_ssn(func_name) {
            Some(_ssn) => _ssn,
            None => {
                return Err(format!("could not find ssn for {:?}", func_name));
            }
        };
        unsafe {
            defs::hNtQueryInformationFileSsn = ssn;
            defs::hNtQueryInformationFileSyscallAddr = syscall_addr;
        }

        let mut iosb = HFile::create_io_stats_block();
        let mut file_information: defs::FILE_INFORMATION::FILE_STANDARD_INFORMATION = defs::FILE_INFORMATION::FILE_STANDARD_INFORMATION {
            AllocationSize: std::ptr::null_mut(),
            EndOfFile: std::ptr::null_mut(),
            NumberOfLinks: 0,
            DeletePending: false,
            Directory: false
        };

        unsafe {
            let ntstatus = defs::hNtQueryInformationFile(
                self.handle,
                &mut iosb,
                &mut file_information as *mut defs::FILE_INFORMATION::FILE_STANDARD_INFORMATION as *mut c_void,
                core::mem::size_of::<defs::FILE_INFORMATION::FILE_STANDARD_INFORMATION>() as u32,
                5
            );
            Ok(ntstatus)
        }
    }
}
