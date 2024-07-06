use std::io::{self, Write}; 
pub fn usr_input(msg:&str) -> String{
    print!("{}", msg);

    let mut input = String::new(); 
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin()
        .read_line(&mut input) // Read a line from standard input and append it to the string
        .expect("Failed to read line");
    
    return input
}


