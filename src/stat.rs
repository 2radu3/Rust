use std::fs;
//use std::os::unix::fs::Metadata;
//use chrono::DateTime;


pub fn stat(arguments: &[String]){
    let file_name = arguments.get(0).unwrap();
    println!("{}",file_name);

    let meta = fs::metadata(file_name).unwrap();  // follows symlinks
    // or:
    // let meta = fs::symlink_metadata("my-symlink")?;

    println!("Size: {} bytes", meta.len());
    //println!("Type: ");
    if meta.is_file(){
        println!("Type: E fisier!!");
    }
        else{
        println!("Type: E director");
        }
    
        

}