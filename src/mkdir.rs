use std::fs;

pub fn mkdir(arguments:&[String]){

    let nume_dir=match arguments.get(0) {
        Some(name)=>name,
        None=>{
            return;
        }
    };
    
let _ =fs::create_dir(nume_dir).unwrap();

}