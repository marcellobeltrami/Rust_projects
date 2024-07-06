#![allow(unused)]
mod functions;

use functions::basics;
use functions::file_man;
use std::io::{stdin,stdout,Write};

fn main() {
    //println!("Hello, world!");
    
    //println!("{}",basics::sum(3, 6));

    //println!("{}", basics::val_checker(0));

    //println!("{}", basics::string_concat("Hello", "there"));

    //let array_1:[&str;5]= ["he","lo","th","th","erre"];

    //println!("{:?}", basics::array_to_vec(array_1));
    
    //println!("{}", basics::colored_fruit(basics::Fruits::Banana));

    // /file_man::file_read("/home/marcello/Rust_projects/practice_rs/test.txt");

    //println!("{:?}",basics::nuc_counter(String::from("TCGACTGACTGATCGATCATCGATGCTAGCTACG")));
    let mut input = String::new();
    print!("Enter an operation: ");
    let _=stdout().flush();
    stdin().read_line(&mut input).expect("Not entered");

    let str_split = input.split("");
    let mut list_inp:Vec<&str> = vec![];
    for inp in str_split{
        list_inp.push(inp)
    }

    let num1:f64 = list_inp[1].parse().unwrap();
    let num2:f64 = list_inp[3].parse().unwrap();


}
