use std::fmt::Error;
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

pub struct HFile<'a> {
    handle: &'a mut HANDLE,
    dll_base: *mut c_void,
    offset: c_longlong,
    file_path: PathBuf,
    mode: defs::hFILE_ACCESS::HFileAccessRights
}

impl<'a> HFile<'a> {
    fn __peb_traverse()  -> Option<*mut c_void> {
        let __ntdll = "ntdll.dll";
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
                    println!("Looped back to the start.");
                    break;
                }

                let _module_name =  match ldr_data_struct.FullDllName.Buffer.to_string() {
                    Ok(_str) => {
                        println!("dll_base: {:#x?}", ldr_data_struct.DllBase);
                        _str
                    },
                    Err(_) => continue
                };

                if _module_name.ends_with(__ntdll) || _module_name == __ntdll {
                    return Some(ldr_data_struct.DllBase);
                }
            }
        }
        return None;
    }

    fn __find_nt_function_address(&self, func_djb2: u64) -> Option<*const c_void> {
        unsafe {
            let dos_header = std::ptr::read(self.dll_base as *const IMAGE_DOS_HEADER);
            if  dos_header.e_magic != IMAGE_DOS_SIGNATURE {
                return None;
            } 
            
            let nt_headers = std::ptr::read(
                self.dll_base.offset(dos_header.e_lfanew as isize) as *const IMAGE_NT_HEADERS64);
            if nt_headers.Signature != IMAGE_NT_SIGNATURE {
                return None;
            }

            let export_directory: IMAGE_EXPORT_DIRECTORY = std::ptr::read(
                self.dll_base.add(nt_headers.OptionalHeader.DataDirectory[0].VirtualAddress as usize) as *const IMAGE_EXPORT_DIRECTORY);

            let functions = self.dll_base.add(export_directory.AddressOfFunctions as usize) as *const u32;
            let names = self.dll_base.add(export_directory.AddressOfNames as usize) as *const u32;
            let ordinals = self.dll_base.add(export_directory.AddressOfNameOrdinals as usize) as *const u16;

            for i in 0.. export_directory.NumberOfNames {
                let function_name = {
                    let __name = self.dll_base.add(*names.offset(i as isize) as usize) as *const u8;
                    let mut len = 0;
                    while *__name.add(len) != 0 {
                        len += 1;
                    }
                    std::slice::from_raw_parts(__name, len)
                };
                let func_name_from_bytes = str::from_utf8(function_name).unwrap();
                
                if utils::djb2(func_name_from_bytes) == func_djb2 {
                    let ordinal = *ordinals.offset(i.try_into().unwrap()) as usize;
                    let fn_addr = self.dll_base.add(
                        *functions.offset(ordinal as isize)  as usize) as *const c_void;
                        
                    return Some(fn_addr);
                }
            }
        }
        None
    }

    fn __find_nt_function_ssn(&self, func_djb2: u64) -> Option<u32> {


        let func_addr = match self.__find_nt_function_address(func_djb2) {
            Some(fa) => fa,
            None => {
                return None;
            }
        };

        return Some(unsafe { *((func_addr as *const u8).add(4) as *const u32) });
}

    fn __init_ntdll_base_functions(&mut self) {
        self.dll_base = match HFile::__peb_traverse() {
            Some(_d) => _d,
            None => {
                panic!("Could not find dll base");
            }
        };
    }

    pub fn new(__handle: &'a mut HANDLE, file_path: PathBuf, mode: defs::hFILE_ACCESS::HFileAccessRights) -> Result<Self, Error> {
        let mut _self: HFile = Self {
            handle: __handle,
            offset: 0,
            file_path,
            mode,
            dll_base: std::ptr::null_mut()
        };
        _self.__init_ntdll_base_functions();
        Ok(_self)
    }

    pub fn get_handle(&self) -> HANDLE {
        *self.handle
    }

    pub fn create(&mut self) -> Result<NTSTATUS, String> {
        let func_name = "NtCreateFile";
        let ssn = match self.__find_nt_function_ssn( utils::djb2(func_name)) {
            Some(_ssn) => _ssn,
            None => {
                return Err(format!("could not find ssn for {:?}", func_name));
            }
        };
        let file_oath_str = self.file_path.to_str().unwrap();

        let mut iosb: IO_STATUS_BLOCK = IO_STATUS_BLOCK {
            Anonymous:  IO_STATUS_BLOCK_0 {
                Status: NTSTATUS(0),
            },
            Information: 0,
        };

        let w_string: widestring::U16CString = widestring::WideCString::from_str(&file_oath_str).unwrap();
        let mut unicode = UNICODE_STRING {
            Length: (file_oath_str.len() * 2) as u16,
            MaximumLength: (file_oath_str.len() * 2) as u16 + 2,
            Buffer: PWSTR::from_raw(w_string.as_ptr() as *mut u16),
        };
        
        let mut obj_attrs = OBJECT_ATTRIBUTES {
            Length: std::mem::size_of::<OBJECT_ATTRIBUTES>() as u32,
            RootDirectory: HANDLE(0),
            ObjectName: &mut unicode,
            Attributes: defs::hOBJECT_ATTRIBUTES::OBJ_CASE_INSENSITIVE,
            SecurityDescriptor: std::ptr::null(),
            SecurityQualityOfService: std::ptr::null(),
        }; 
        unsafe {
            defs::hNtCreateFileSsn = ssn;
            println!("before handle: {:?}", self.handle as *mut HANDLE);
            let ntstatus =  defs::hNtCreateFile(
                self.handle as *mut HANDLE,
                self.mode,
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
            println!("handle: {:?}", self.handle as *mut HANDLE);
            Ok(ntstatus)
        }
        
    }

    pub fn write(&mut self, mut buffer: Vec<u8>) -> Result<(NTSTATUS, usize), String> {
        let func_name = hide!("NtWriteFile");
        let ssn = match self.__find_nt_function_ssn(func_name) {
            Some(_ssn) => _ssn,
            None => {
                return Err(format!("could not find ssn for {:?}", func_name));
            }
        };
        let mut iosb: IO_STATUS_BLOCK = IO_STATUS_BLOCK {
            Anonymous:  IO_STATUS_BLOCK_0 {
                Status: NTSTATUS(0),
            },
            Information: 0,
        };        

        unsafe {
            println!("handle in: {:?}",  self.handle as *const HANDLE);
            defs::hNtWriteFileSsn = ssn;
            let __status = defs::hNtWriteFile(
                *self.handle, 
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

    // pub fn open(__handle: &'a mut HANDLE, file_path: PathBuf, mode: defs::hFILE_ACCESS::HFileAccessRights) -> Result<Self, Error> {
    //     let mut _self = Self {
    //         handle: __handle,
    //         offset: 0,
    //         file_path,
    //         mode,
    //         dll_base: std::ptr::null_mut()
    //     };

    //     _self.__init_ntdll_base_functions();
    //     Ok(_self)
    // }


}


