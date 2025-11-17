use std::fs;
use std::path::Path;

pub fn mv(arguments: &[String]) {
    if arguments.len() < 2 {
        eprintln!("mv: lipsesc operanduri");
        eprintln!("Utilizare: mv <sursa> <destinatie>");
        return;
    }

    let src = &arguments[0];
    let dst = &arguments[1];

    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    if !src_path.exists() {
        eprintln!("mv: nu pot accesa '{}': fișier inexistent", src);
        return;
    }

    let final_dst = if dst_path.is_dir() {
        match src_path.file_name() {
            Some(name) => dst_path.join(name),
            None => {
                eprintln!("mv: eroare la extragerea numelui din sursă");
                return;
            }
        }
    } else {
        dst_path.to_path_buf()
    };


    if let Err(e) = fs::rename(src_path, &final_dst) {
        eprintln!("mv: eroare la mutare sau redenumire: {}", e);
    }
}
