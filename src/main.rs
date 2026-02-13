use std::arch::asm;
use std::ffi::{CString, OsString, c_longlong, c_ulong, c_void};
use std::ops::Sub;
use std::ptr::{null, null_mut};
use std::{panic, slice};
use windows_core::PCSTR;

use windows::Win32::Foundation::{HANDLE, NTSTATUS, UNICODE_STRING};
use windows::Win32::Security::{SECURITY_DESCRIPTOR, SECURITY_QUALITY_OF_SERVICE};
use windows::Win32::System::Diagnostics::Debug::{IMAGE_NT_HEADERS64, OBJECT_ATTRIB_FLAGS};
use windows::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_EXPORT_DIRECTORY, IMAGE_NT_SIGNATURE};
use windows::Win32::System::Threading::{PEB, PEB_LDR_DATA};
use windows::Win32::System::WindowsProgramming::{LDR_DATA_TABLE_ENTRY};
use windows::core::PWSTR;

#[unsafe(no_mangle)]
pub static mut hNtCreateFileSsn: u32 = 0;
#[unsafe(no_mangle)]
pub static mut hNtWriteFileSsn: u32 = 0;

unsafe extern "C" {
    fn hNtCreateFile(
        file_handle: *mut HANDLE,
        desired_access: u32,
        object_attributes: *mut OBJECT_ATTRIBUTES,
        io_status_block: *mut IO_STATUS_BLOCK,
        allocation_size: *mut LARGE_INTEGER,
        file_attributes: u32,
        share_access: u32,
        create_disposition: u32,
        create_options: u32,
        ea_buffer: *mut c_void,
        ea_length: u32,
    ) -> NTSTATUS;

    fn hNtWriteFile(
        file_handle: HANDLE,
        event: *mut HANDLE,
        apc_routine: *mut c_void,
        apc_context: *mut c_void,
        io_status_block: *mut IO_STATUS_BLOCK,
        buffer: *const u8,
        length: usize,
        offset: *mut LARGE_INTEGER,
        key: u32,
    ) -> NTSTATUS;
}

fn nt_create_file(
    file_name: &str,
    handle: &mut HANDLE,
    ssn: u32
) -> NTSTATUS {

    let mut iosb = IO_STATUS_BLOCK {
        status: NTSTATUS(0),
        information: 0,
        ponter: std::ptr::null_mut()
    };
    let w_string: widestring::U16CString = widestring::WideCString::from_str(&file_name).unwrap();
    let mut unicode = UNICODE_STRING {
        Length: (file_name.len() * 2) as u16,
        MaximumLength: (file_name.len() * 2) as u16 + 2,
        Buffer: PWSTR::from_raw(w_string.as_ptr() as *mut u16),
    };

    unsafe {println!("unicode file name: {:?}", unicode.Buffer.to_hstring().unwrap());}

    let mut obj_attrs = OBJECT_ATTRIBUTES {
        length: std::mem::size_of::<OBJECT_ATTRIBUTES>() as u32,
        root_directory: HANDLE(0),
        object_name: &mut unicode,
        attributes: OBJECT_ATTRIB_FLAGS(64), // OBJ_CASE_INSENSITIVE
        security_descriptor: std::ptr::null(),
        security_quality_of_service: std::ptr::null(),
    };  

    unsafe {
        hNtCreateFileSsn = ssn;

        let __status = hNtCreateFile(
            handle,
        0x00100000 | 0x40000000,
        &mut obj_attrs,
        &mut iosb,
        std::ptr::null_mut(),
        0x00000080,
        0x00000001,
        0x00000005,
        0x00000020,
        std::ptr::null_mut(),
        0,
        );
        println!("iosb: {:?}", iosb.status);
        __status
    }
}

fn nt_write_file(file_handle: HANDLE, mut buffer: Vec<u8>, offset: u32, ssn: u32) -> NTSTATUS {
    let mut iosb = IO_STATUS_BLOCK {
        status: NTSTATUS(0),
        information: 0,
        ponter: std::ptr::null_mut()
    };

    let mut __offset = LARGE_INTEGER {
        quad_part: offset as i64
    };
    unsafe {
        hNtWriteFileSsn = ssn;
        let __status = hNtWriteFile(
            file_handle, 
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut iosb,
            buffer.as_mut_ptr() as *const u8,
            buffer.len(),
            &mut __offset,
            0
        );
        __status
    }
}

fn peb_traverse(module_name: &str) -> Option<*mut c_void> {
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

            if _module_name.ends_with(module_name) || _module_name == module_name {
                return Some(ldr_data_struct.DllBase);
            }
        }
    }
    return None;
}

fn main() {
    let nt_create_file_ssn = match find_nt_function_ssn("NtCreateFile") {
        Some(_ssn) => _ssn,
        None => {
            panic!("NtCreateFile ssn not found");
        }
    };
    let mut handle: HANDLE = HANDLE(0);
    let fname: &str = "\\??\\C:\\temp\\hell_test.txt";
    let new_file = nt_create_file(
        fname,
            &mut handle,
            nt_create_file_ssn);

    println!("file handle: {:?}", new_file);

     let nt_write_file_ssn = match find_nt_function_ssn("NtWriteFile") {
        Some(_ssn) => _ssn,
        None => {
            panic!("NtCreateFile ssn not found");
        }
    };

    let file_string = String::from("value");

    let status = nt_write_file(
        handle,
        file_string.into_bytes(),
        0,
        nt_write_file_ssn);
    println!("status: {:#x?}", status);

}
    
fn find_nt_function_address(dll_base: *mut c_void, func_name: &str) -> Option<*const c_void> {
    unsafe {
        
        let dos_header = std::ptr::read(dll_base as *const IMAGE_DOS_HEADER);
        if  dos_header.e_magic != IMAGE_DOS_SIGNATURE {
            return None;
        } 
        
        let nt_headers = std::ptr::read(
            dll_base.offset(dos_header.e_lfanew as isize) as *const IMAGE_NT_HEADERS64);
        if nt_headers.Signature != IMAGE_NT_SIGNATURE {
            return None;
        }

        let export_directory: IMAGE_EXPORT_DIRECTORY = std::ptr::read(
            dll_base.add(nt_headers.OptionalHeader.DataDirectory[0].VirtualAddress as usize) as *const IMAGE_EXPORT_DIRECTORY);
        // println!("dll_base:     {:#x?}", dll_base);
        // println!("adding  :     {:#x?}", nt_headers.OptionalHeader.DataDirectory[0].VirtualAddress as usize);
        // println!("dll_base + V: {:#x?}", dll_base.add(nt_headers.OptionalHeader.DataDirectory[0].VirtualAddress as usize));
        let functions = dll_base.add(export_directory.AddressOfFunctions as usize) as *const u32;
        let names = dll_base.add(export_directory.AddressOfNames as usize) as *const u32;
        let ordinals = dll_base.add(export_directory.AddressOfNameOrdinals as usize) as *const u16;

        let number_of_names = export_directory.NumberOfNames;
        for i in 0..number_of_names {
            let function_name = {
                let __name = dll_base.add(*names.offset(i as isize) as usize) as *const u8;
                let mut len = 0;
                while *__name.add(len) != 0 {
                    len += 1;
                }
                std::slice::from_raw_parts(__name, len)
            };
            let func_name_from_bytes = str::from_utf8(function_name).unwrap();
            
            if func_name_from_bytes == func_name {
                let ordinal = *ordinals.offset(i.try_into().unwrap()) as usize;
                let fn_addr = dll_base.add(
                    *functions.offset(ordinal as isize)  as usize) as *const c_void;
                    
                println!("[i] Function {:?} address: {:p}", func_name_from_bytes, fn_addr);
                return Some(fn_addr);
            }
        }
    }
    None
}
fn find_nt_function_ssn(func_name: &str) -> Option<u32> {
    let dll_base = match peb_traverse("ntdll.dll") {
        Some(_base) => {
            _base
        },
        None => {
            panic!("ntdll not found")
        }
    };

    let func_addr = match find_nt_function_address(dll_base, func_name) {
        Some(fa) => fa,
        None => {
            return None;
        }
    };

    return Some(unsafe { *((func_addr as *const u8).add(4) as *const u32) });
}
#[repr(C)]
#[derive(Debug)]
pub struct OBJECT_ATTRIBUTES {
    pub length: u32,
    pub root_directory: HANDLE,
    pub object_name: *const UNICODE_STRING,
    pub attributes: OBJECT_ATTRIB_FLAGS,
    pub security_descriptor: *const SECURITY_DESCRIPTOR,
    pub security_quality_of_service: *const SECURITY_QUALITY_OF_SERVICE,
}

// #[repr(C)]
// struct CLIENT_ID {
//     unique_proc: *mut c_void,
//     unique_thread: *mut c_void
// }

#[repr(C)]
pub struct IO_STATUS_BLOCK {
    pub status: NTSTATUS,
    pub information: usize,
    pub ponter: *mut c_void 
}

#[repr(C)]
pub struct LARGE_INTEGER {
    pub quad_part: i64,
}

// __kernel_entry NTSTATUS NtCreateFile(
//   [out]          PHANDLE            FileHandle,
//   [in]           ACCESS_MASK        DesiredAccess,
//   [in]           POBJECT_ATTRIBUTES ObjectAttributes,
//   [out]          PIO_STATUS_BLOCK   IoStatusBlock,
//   [in, optional] PLARGE_INTEGER     AllocationSize,
//   [in]           ULONG              FileAttributes,
//   [in]           ULONG              ShareAccess,
//   [in]           ULONG              CreateDisposition,
//   [in]           ULONG              CreateOptions,
//   [in]           PVOID              EaBuffer,
//   [in]           ULONG              EaLength
// );

// __kernel_entry NTSYSCALLAPI NTSTATUS NtWriteFile(
//   [in]           HANDLE           FileHandle,
//   [in, optional] HANDLE           Event,
//   [in, optional] PIO_APC_ROUTINE  ApcRoutine,
//   [in, optional] PVOID            ApcContext,
//   [out]          PIO_STATUS_BLOCK IoStatusBlock,
//   [in]           PVOID            Buffer,
//   [in]           ULONG            Length,
//   [in, optional] PLARGE_INTEGER   ByteOffset,
//   [in, optional] PULONG           Key
// );

// status = NtWriteFile(h,NULL,NULL,NULL, &isb, Buffer, uSize, NULL, NULL);

// fn hNtCreateFile(
//         file_handle: *mut HANDLE,
//         desired_access: u32,
//         object_attributes: *mut OBJECT_ATTRIBUTES,
//         io_status_block: *mut IO_STATUS_BLOCK,
//         allocation_size: *mut LARGE_INTEGER,
//         file_attributes: u32,
//         share_access: u32,
//         create_disposition: u32,
//         create_options: u32,
//         ea_buffer: *mut c_void,
//         ea_length: u32,
//     ) -> NTSTATUS {
//         let ntstatus: i32;
//         unsafe {

//             asm!(
//                 "mov r10, rcx",
//                 "mov eax, {0:e}",
//                 "syscall",
//                 "ret",
//                 in(reg) hNtCreateFileSsn,
//                 lateout("rax") ntstatus
//             );

//             NTSTATUS(ntstatus as i32)
//         }
//     }

//     fn hNtWriteFile(
//         file_handle: *const HANDLE,
//         event: *mut HANDLE,
//         apc_routine: *mut c_void,
//         apc_context: *mut c_void,
//         io_status_block: *mut IO_STATUS_BLOCK,
//         buffer: *const u8,
//         length: usize,
//         offset: *mut LARGE_INTEGER,
//         key: u32,
//     ) -> NTSTATUS {
//         let ntstatus: i32;
//         unsafe {

//             asm!(
//                 "mov r10, rcx",
//                 "mov eax, {0:e}",
//                 "syscall",
//                 "ret",
//                 in(reg) hNtWriteFileSsn,
//                 lateout("rax") ntstatus
//             );

//             NTSTATUS(ntstatus as i32)
//         }
//     }