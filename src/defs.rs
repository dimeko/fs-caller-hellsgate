#![allow(non_snake_case)]

use std::ffi::{c_longlong, c_void};
use windows::Win32::System::IO::IO_STATUS_BLOCK;
use windows::Wdk::Foundation::OBJECT_ATTRIBUTES;
use windows::Win32::Foundation::{HANDLE, NTSTATUS};

pub const DW_HASH: u64 = 0x7734773477347734;
// File Access Flags -----------------------------------------------------------------------------------
pub mod hFILE_ACCESS {
    pub type HFileAccessRights = u32;
    pub const FILE_READ_DATA: HFileAccessRights            = 0x0001; // file & pipe
    pub const FILE_LIST_DIRECTORY: HFileAccessRights       = 0x0001; // directory
    pub const FILE_WRITE_DATA: HFileAccessRights           = 0x0002; // file & pipe
    pub const FILE_ADD_FILE: HFileAccessRights             = 0x0002; // directory
    pub const FILE_APPEND_DATA: HFileAccessRights          = 0x0004; // file
    pub const FILE_ADD_SUBDIRECTORY: HFileAccessRights     = 0x0004; // directory
    pub const FILE_CREATE_PIPE_INSTANCE: HFileAccessRights = 0x0004; // named pipe
    pub const FILE_READ_EA: HFileAccessRights              = 0x0008; // file & directory
    pub const FILE_WRITE_EA: HFileAccessRights             = 0x0010; // file & directory
    pub const FILE_EXECUTE: HFileAccessRights              = 0x0020; // file
    pub const FILE_TRAVERSE: HFileAccessRights             = 0x0020; // directory
    pub const FILE_DELETE_CHILD: HFileAccessRights         = 0x0040; // directory
    pub const FILE_READ_ATTRIBUTES: HFileAccessRights      = 0x0080; // all
    pub const FILE_WRITE_ATTRIBUTES: HFileAccessRights     = 0x0100; // all
    
    // Composite Access Rights
    pub const STANDARD_RIGHTS_REQUIRED: u32 = 0x000F0000;
    pub const STANDARD_RIGHTS_READ: u32     = 0x00020000;
    pub const STANDARD_RIGHTS_WRITE: u32    = 0x00020000;
    pub const STANDARD_RIGHTS_EXECUTE: u32  = 0x00020000;
    pub const SYNCHRONIZE: u32              = 0x00100000;
    
    pub const FILE_ALL_ACCESS: u32 =
    STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0x01FF;
        
    pub const FILE_GENERIC_READ: u32 =
        STANDARD_RIGHTS_READ |
        FILE_READ_DATA |
        FILE_READ_ATTRIBUTES |
        FILE_READ_EA |
        SYNCHRONIZE;

    pub const FILE_GENERIC_WRITE: u32 =
    STANDARD_RIGHTS_WRITE |
        FILE_WRITE_DATA |
        FILE_WRITE_ATTRIBUTES |
        FILE_WRITE_EA |
        FILE_APPEND_DATA |
        SYNCHRONIZE;

    pub const FILE_GENERIC_EXECUTE: u32 =
        STANDARD_RIGHTS_EXECUTE |
        FILE_READ_ATTRIBUTES |
        FILE_EXECUTE |
        SYNCHRONIZE;
}

// (END) File Access Flags -----------------------------------------------------------------------------------

// OBJECT_ATTRIBUTES > Attributes
pub mod hOBJECT_ATTRIBUTES {
    pub const OBJ_CASE_INSENSITIVE: u32 = 0x00000040;
}

pub mod hFILE_ATTRIBUTES {
    pub const OBJECT_ATTRIBUTES_READONLY: u32 =        0x00000001;
    pub const FILE_ATTRIBUTE_HIDDEN: u32 =             0x00000002;
    pub const FILE_ATTRIBUTE_SYSTEM: u32 =             0x00000004;
    pub const FILE_ATTRIBUTE_DIRECTORY: u32 =          0x00000010;
    pub const FILE_ATTRIBUTE_ARCHIVE: u32 =            0x00000020;
    pub const FILE_ATTRIBUTE_DEVICE: u32 =             0x00000040;
    pub const FILE_ATTRIBUTE_NORMAL: u32 =             0x00000080;
    pub const FILE_ATTRIBUTE_TEMPORARY: u32 =          0x00000100;
    pub const FILE_ATTRIBUTE_SPARSE_FILE: u32 =        0x00000200;
    pub const FILE_ATTRIBUTE_REPARSE_POINT: u32 =      0x00000400;
    pub const FILE_ATTRIBUTE_COMPRESSED: u32 =         0x00000800;
    pub const FILE_ATTRIBUTE_OFFLINE: u32 =            0x00001000;
    pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: u32 = 0x00002000;
    pub const FILE_ATTRIBUTE_ENCRYPTED: u32 =          0x00004000;
    pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: u32 =   0x00008000;
}

// Share Access Flags
pub mod hSHARE_ACCESS {
    pub const FILE_SHARE_READ: u32   = 0x01;
    pub const FILE_SHARE_WRITE: u32  = 0x02;
    pub const FILE_SHARE_DELETE: u32 = 0x04;    
}

// NtCreateFile Result Values
pub const FILE_SUPERSEDED: u32     = 0x00000000;
pub const FILE_OPENED: u32         = 0x00000001;
pub const FILE_CREATED: u32        = 0x00000002;
pub const FILE_OVERWRITTEN: u32    = 0x00000003;
pub const FILE_EXISTS: u32         = 0x00000004;
pub const FILE_DOES_NOT_EXIST: u32 = 0x00000005;


// CreateDisposition (OpenType)

pub mod hCREATE_DISPOSITION {
    pub const FILE_SUPERSEDE: u32           = 0x00000000;
    pub const FILE_OPEN: u32                = 0x00000001;
    pub const FILE_CREATE: u32              = 0x00000002;
    pub const FILE_OPEN_IF: u32             = 0x00000003;
    pub const FILE_OVERWRITE: u32           = 0x00000004;
    pub const FILE_OVERWRITE_IF: u32        = 0x00000005;
}
    
// CreateOptions Flags
pub mod hCREATE_OPTIONS {
    pub const FILE_DIRECTORY_FILE: u32            = 0x00000001;
    pub const FILE_WRITE_THROUGH: u32             = 0x00000002;
    pub const FILE_SEQUENTIAL_ONLY: u32           = 0x00000004;
    pub const FILE_NO_INTERMEDIATE_BUFFERING: u32 = 0x00000008;
    pub const FILE_SYNCHRONOUS_IO_ALERT: u32      = 0x00000010;
    pub const FILE_SYNCHRONOUS_IO_NONALERT: u32   = 0x00000020;
    pub const FILE_NON_DIRECTORY_FILE: u32        = 0x00000040;
    pub const FILE_CREATE_TREE_CONNECTION: u32    = 0x00000080;
    pub const FILE_COMPLETE_IF_OPLOCKED: u32      = 0x00000100;
    pub const FILE_NO_EA_KNOWLEDGE: u32           = 0x00000200;
    pub const FILE_OPEN_REMOTE_INSTANCE: u32      = 0x00000400;
    pub const FILE_RANDOM_ACCESS: u32             = 0x00000800;
    pub const FILE_DELETE_ON_CLOSE: u32           = 0x00001000;
    pub const FILE_OPEN_BY_FILE_ID: u32           = 0x00002000;
    pub const FILE_OPEN_FOR_BACKUP_INTENT: u32    = 0x00004000;
    pub const FILE_NO_COMPRESSION: u32            = 0x00008000;
    pub const FILE_RESERVE_OPFILTER: u32          = 0x00100000;
    pub const FILE_OPEN_REPARSE_POINT: u32        = 0x00200000;
    pub const FILE_OPEN_NO_RECALL: u32            = 0x00400000;
    pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: u32 = 0x00800000;
}

pub const FILE_USE_FILE_POINTER_POSITION: u32 = 0xFFFFFFFE;
pub const FILE_WRITE_TO_END_OF_FILE: u32      = 0xFFFFFFFF;

#[unsafe(no_mangle)]
#[warn(non_upper_case_globals)]
pub static mut hNtCreateFileSsn: u32 = 0;
#[unsafe(no_mangle)]
pub static mut hNtWriteFileSsn: u32 = 0;
#[unsafe(no_mangle)]
pub static mut hNtOpenFileSsn: u32 = 0;
#[unsafe(no_mangle)]
pub static mut hNtCloseFileSsn: u32 = 0;
#[unsafe(no_mangle)]
pub static mut hNtReadFileSsn: u32 = 0;
#[unsafe(no_mangle)]
pub static mut hNtDeleteFileSsn: u32 = 0;
#[unsafe(no_mangle)]
pub static mut hNtQueryInformationFileSsn: u32 = 0;

unsafe extern "C" {
    pub fn hNtCreateFile(
        file_handle: *mut HANDLE,
        desired_access: u32,
        object_attributes: *mut OBJECT_ATTRIBUTES,
        io_status_block: *mut IO_STATUS_BLOCK,
        allocation_size: *mut c_longlong,
        file_attributes: u32,
        share_access: u32,
        create_disposition: u32,
        create_options: u32,
        ea_buffer: *mut c_void,
        ea_length: u32,
    ) -> NTSTATUS;

    pub fn hNtWriteFile(
        file_handle: HANDLE,
        event: *mut HANDLE,
        apc_routine: *mut c_void,
        apc_context: *mut c_void,
        io_status_block: *mut IO_STATUS_BLOCK,
        buffer: *const u8,
        length: usize, // here maybe it is u32 type
        offset: *mut c_longlong,
        key: u32,
    ) -> NTSTATUS;

    pub fn hNtReadFile(
        file_handle: HANDLE,
        event: *mut HANDLE,
        apc_routine: *mut c_void,
        apc_context: *mut c_void,
        io_status_block: *mut IO_STATUS_BLOCK,
        buffer: *const u8,
        length: u32,
        offset: *mut c_longlong,
        key: u32,
    ) -> NTSTATUS;

    pub fn hNtOpenFile(
        file_handle: *mut HANDLE,
        desired_access: u32,
        object_attributes: *mut OBJECT_ATTRIBUTES,
        io_status_block: *mut IO_STATUS_BLOCK,
        share_access: u32,
        open_options: u32,
    ) -> NTSTATUS;

    pub fn hNtCloseFile(
        file_handle: HANDLE,
    ) -> NTSTATUS;

    pub fn hNtDeleteFile(
        object_attributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;

    pub fn hNtQueryInformationFile(
        file_handle: HANDLE,
        io_status_block: *mut IO_STATUS_BLOCK,
        file_information: *mut c_void,
        length: u32,
        file_information_class: u32,
    ) -> NTSTATUS;
}