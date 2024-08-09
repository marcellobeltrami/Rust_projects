use std::io::{self, Write}; 
pub fn usr_input(msg:&str) -> String{
    
    let mut error=1;
    
    while error ==1 {
        
        print!("{}", msg); //Print prompt
        //Gets usr input
        let mut input = String::new(); 
        io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
        io::stdin()
            .read_line(&mut input) // Read a line from standard input and append it to the string
            .expect("Failed to read line");
    
        input= input.trim().to_string();
        
        //Check usr input against allowed commands . Could modify this so if the command is matched, the specific action is executes
        match input.as_str() {
            "y"=>return input,
            "n"=>return input,
            "scan -e"=>return input,
            "scan -c"=>return input,
            "heal"=> return input,
            "charge"=>return input,
            "hack"=> return input,
            _=> {error=1; println!("Invalid command. \nValid commands are: \ny, n \nscan -e, scan -c \nheal, charge, hack"); }
        }

    }
    unreachable!() // This line should never be reached, throwa 
}



