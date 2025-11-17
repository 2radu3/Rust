use std::env;

pub fn pwd(_arguments: &[String]){
    let dir_name=env::current_dir().unwrap();
    println!("{}", dir_name.display());
}