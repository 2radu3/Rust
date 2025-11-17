use std::fs;
use std::path::Path;

pub fn mv(arguments: &[String]) {
    // Trebuie minim două argumente: sursa și destinația
    if arguments.len() < 2 {
        eprintln!("mv: lipsesc operanduri");
        eprintln!("Utilizare: mv <sursa> <destinatie>");
        return;
    }

    let src = &arguments[0];
    let dst = &arguments[1];

    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    // Verificăm dacă sursa există
    if !src_path.exists() {
        eprintln!("mv: nu pot accesa '{}': fișier inexistent", src);
        return;
    }

    // Dacă destinația este un director → mutăm în interiorul lui
    let final_dst = if dst_path.is_dir() {
        // concatenează numele original
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

    // Mutăm / redenumim
    if let Err(e) = fs::rename(src_path, &final_dst) {
        eprintln!("mv: eroare la mutare sau redenumire: {}", e);
    }
}