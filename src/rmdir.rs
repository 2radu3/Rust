use std::fs;

pub fn rmdir(arguments:&[String]){

    let nume_dir=match arguments.get(0) {
        Some(name)=>name,
        None=>{
            return;
        }
    };
    
let _ =fs::remove_dir(nume_dir).unwrap();

}