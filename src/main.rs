use std::path::PathBuf;
use hellsgate::{HFile, defs};

fn main() {
    let file_path: &str = "\\??\\C:\\temp\\test.txt";
    let  (mut file, ntstatus) = match HFile::create(
        PathBuf::from(file_path),
        defs::hFILE_ACCESS::FILE_GENERIC_WRITE) {
        Ok(_f) => _f,
        Err(_e) => {
            panic!("Error creating file object: {}", _e);
        }
    };
    println!("create status: {:#x?}", ntstatus);

    let input_bytes = String::from("written bytes in the test file");
    let (ntstatus, _) = file.write(input_bytes.into_bytes()).unwrap();
    println!("write status: {:#x?}", ntstatus);

    let ntstatus = file.close().unwrap();
    println!("close status: {:#x?}", ntstatus);

    let (mut file, ntstatus) = match HFile::open(
        PathBuf::from(file_path),
        defs::hFILE_ACCESS::FILE_GENERIC_READ) {
        Ok(_f) => _f,
        Err(_e) => {
            panic!("Error opening file object: {}", _e);
        }
    };
    println!("open status: {:#x?}", ntstatus);

    let (ntstatus, bytes_read) = match file.read(2) {
        Ok(_b) => _b,
        Err(_e) => {
            panic!("could not read bytes: {:?}", _e);
        }
    };
    println!("read status: {:#x?}", ntstatus);
    println!("bytes read: {:?}", bytes_read);

    // let _ = file.info();
}