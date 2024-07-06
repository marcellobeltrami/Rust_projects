use std::env;
use std::fs;

//Reads a file and prints its contents
pub fn file_read(file_path:&str) {
    let contents = fs::read_to_string(file_path)
    .expect("File not found");
    
    println!("{contents}\n");

}