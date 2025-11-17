use std::env;

pub fn env (_arguments: &[String]){
    for(key, value) in env::vars(){
        println!("{key}: {value}");
    }
}