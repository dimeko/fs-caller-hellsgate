// use std::arch::asm;
// use std::ffi::{CString, OsString, c_longlong, c_ulong, c_void};
// use std::ops::Sub;
// use std::ptr::{null, null_mut};
// use std::{panic, slice};
// use windows_core::PCSTR;

// use windows::Win32::Foundation::{HANDLE, NTSTATUS, UNICODE_STRING};
// use windows::Win32::Security::{SECURITY_DESCRIPTOR, SECURITY_QUALITY_OF_SERVICE};
// use windows::Win32::System::Diagnostics::Debug::{IMAGE_NT_HEADERS64, OBJECT_ATTRIB_FLAGS};
// use windows::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_EXPORT_DIRECTORY, IMAGE_NT_SIGNATURE};
// use windows::Win32::System::Threading::{PEB, PEB_LDR_DATA};
// use windows::Win32::System::WindowsProgramming::{LDR_DATA_TABLE_ENTRY};
// use windows::core::PWSTR;

// // #[unsafe(no_mangle)]
// // pub static mut hNtCreateFileSsn: u32 = 0;
// // #[unsafe(no_mangle)]
// // pub static mut hNtWriteFileSsn: u32 = 0;

// // unsafe extern "C" {
// //     fn hNtCreateFile(
// //         file_handle: *mut HANDLE,
// //         desired_access: u32,
// //         object_attributes: *mut OBJECT_ATTRIBUTES,
// //         io_status_block: *mut IO_STATUS_BLOCK,
// //         allocation_size: *mut LARGE_INTEGER,
// //         file_attributes: u32,
// //         share_access: u32,
// //         create_disposition: u32,
// //         create_options: u32,
// //         ea_buffer: *mut c_void,
// //         ea_length: u32,
// //     ) -> NTSTATUS;

// //     fn hNtWriteFile(
// //         file_handle: HANDLE,
// //         event: *mut HANDLE,
// //         apc_routine: *mut c_void,
// //         apc_context: *mut c_void,
// //         io_status_block: *mut IO_STATUS_BLOCK,
// //         buffer: *const u8,
// //         length: usize, // here maybe it is u32 type
// //         offset: *mut LARGE_INTEGER,
// //         key: u32,
// //     ) -> NTSTATUS;

// //     fn hNtReadFile(
// //         file_handle: HANDLE,
// //         event: *mut HANDLE,
// //         apc_routine: *mut c_void,
// //         apc_context: *mut c_void,
// //         io_status_block: *mut IO_STATUS_BLOCK,
// //         buffer: *const u8,
// //         length: u32,
// //         offset: *mut LARGE_INTEGER,
// //         key: u32,
// //     ) -> NTSTATUS;

// //     fn hNtOpenFile(
// //         file_handle: *mut HANDLE,
// //         desired_access: u32,
// //         object_attributes: *mut OBJECT_ATTRIBUTES,
// //         io_status_block: *mut IO_STATUS_BLOCK,
// //         share_access: u32,
// //         open_options: u32,
// //     ) -> NTSTATUS;

// //     fn hNtCloseFile(
// //         file_handle: HANDLE,
// //     ) -> NTSTATUS;

// //     fn hNtDeleteFile(
// //         object_attributes: *mut OBJECT_ATTRIBUTES,
// //     ) -> NTSTATUS;

// //     fn hNtQueryInformationFile(
// //         file_handle: HANDLE,
// //         io_status_block: *mut IO_STATUS_BLOCK,
// //         file_information: *mut c_void,
// //         length: u32,
// //         file_information_class: u32,
// //     ) -> NTSTATUS;
// // }

// fn nt_write_file(file_handle: HANDLE, mut buffer: Vec<u8>, offset: u32, ssn: u32) -> NTSTATUS {
//     let mut __offset = LARGE_INTEGER {
//         quad_part: offset as i64
//     };
//     unsafe {
//         hNtWriteFileSsn = ssn;
//         let __status = hNtWriteFile(
//             file_handle, 
//             std::ptr::null_mut(),
//             std::ptr::null_mut(),
//             std::ptr::null_mut(),
//             &mut iosb,
//             buffer.as_mut_ptr() as *const u8,
//             buffer.len(),
//             &mut __offset,
//             0
//         );
//         __status
//     }
// }


// fn main() {
//     let nt_create_file_ssn = match find_nt_function_ssn("NtCreateFile") {
//         Some(_ssn) => _ssn,
//         None => {
//             panic!("NtCreateFile ssn not found");
//         }
//     };
//     let mut handle: HANDLE = HANDLE(0);
//     let fname: &str = "\\??\\C:\\temp\\hell_test.txt";
//     let new_file = nt_create_file(
//         fname,
//             &mut handle,
//             nt_create_file_ssn);

//     println!("file handle: {:?}", new_file);

//      let nt_write_file_ssn = match find_nt_function_ssn("NtWriteFile") {
//         Some(_ssn) => _ssn,
//         None => {
//             panic!("NtCreateFile ssn not found");
//         }
//     };

//     let file_string = String::from("value");

//     let status = nt_write_file(
//         handle,
//         file_string.into_bytes(),
//         0,
//         nt_write_file_ssn);
//     println!("status: {:#x?}", status);

// }


use std::path::PathBuf;
use hellsgate::{HFile, defs};
use windows::Win32::Foundation::HANDLE;

fn main() {
    let mut handle= HANDLE(0);
    println!("init handle: {:?}", &handle as *const HANDLE);
    let file_path: &str = "\\??\\C:\\temp\\in_lib.txt";
    let mut file = match HFile::new(
        &mut handle,
        PathBuf::from(file_path),
        defs::hFILE_ACCESS::FILE_GENERIC_WRITE) {
        Ok(_f) => _f,
        Err(_e) => {
            panic!("Error creating file object: {}", _e);
        }
    };

    println!("file handle: {:?}", file.get_handle());

    let file_string = String::from("value");
    let _ = file.create();
    println!("File was created. Writing ...");
    let r = file.write(file_string.into_bytes()).unwrap();


    println!("status: {:#x?}", r.0);

}