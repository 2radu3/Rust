use std::fs;
use std::os::unix::fs::MetadataExt; // pentru uid, gid, atime, mtime, ctime
use chrono::{DateTime, Local};

pub fn stat(arguments: &[String]) {
    let file_name = match arguments.get(0) {
        Some(name) => name,
        None => {
            eprintln!("stat: lipseste numele fisierului");
            return;
        }
    };

    let metadata = match fs::symlink_metadata(file_name) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("stat: eroare la citirea '{}': {}", file_name, e);
            return;
        }
    };

    let file_type = if metadata.is_dir() {
        "Directory"
    } else if metadata.is_file() {
        "File"
    } else if metadata.file_type().is_symlink() {
        "Symlink"
    } else {
        "Other"
    };

    let permissions = metadata.mode() & 0o777;
    
    let uid = metadata.uid();
    let gid = metadata.gid();

    let size = metadata.len();

    let access_time = DateTime::<Local>::from(
        std::time::UNIX_EPOCH + std::time::Duration::from_secs(metadata.atime() as u64)
    );
    let modify_time = DateTime::<Local>::from(
        std::time::UNIX_EPOCH + std::time::Duration::from_secs(metadata.mtime() as u64)
    );
    let change_time = DateTime::<Local>::from(
        std::time::UNIX_EPOCH + std::time::Duration::from_secs(metadata.ctime() as u64)
    );

    println!("File: {}", file_name);
    println!("Type: {}", file_type);
    println!("Permissions: {:o}", permissions);
    println!("Owner: UID={} GID={}", uid, gid);
    println!("Size: {} bytes", size);
    println!("Accessed: {}", access_time);
    println!("Modified: {}", modify_time);
    println!("Changed: {}", change_time);
}
