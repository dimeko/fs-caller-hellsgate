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

    println!("creation status: {:#x?}", ntstatus.0);

    println!("file handle: {:?}", file.get_handle());
    let input_bytes = String::from("vadlue");
    println!("File was created. Writing ...");
    let r = file.write(input_bytes.into_bytes()).unwrap();


    println!("status: {:#x?}", r.0);
    file.info();

}