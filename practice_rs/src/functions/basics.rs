use std::{collections::HashMap, iter::Enumerate};


// Returns summed value
pub fn sum(a:i32, b:i32)->i32{
    a+b
}
 
// Returns checks negative/positive
pub fn val_checker(a:i32) -> String{

    if (a < 0){
        return String::from("Negative");
    }
    else if (a>0) {
        return String::from("Positive");
    } else {
        return String::from("Zero");
    }
}


//Concatenate 2 strings
pub fn string_concat(a:&str, b:&str)->String{

    let s_a = String::from(a);
    let s_b = String::from(b);

    let s:String = s_a + &s_b;
    return s;


}

//Converts a 5 items array to a vector
pub fn array_to_vec(list:[&str; 5])-> Vec<&str>{

    let mut array = Vec::new();
    for i in list{
        array.push(i);
    }

    array
}


// Define an enum and return a value based on the inputted enum
pub enum Fruits{
    Banana,
    Apple,

}

pub fn colored_fruit(fruit:Fruits) -> String{

    match fruit {
        Fruits::Apple => String::from("red"),
        Fruits::Banana => String::from("yellow"),
        _ => String::from("Fruit not found")
        
    }

}


//Using a HashMap to count each character in a string. This is applied as a nucleotide counter.
pub fn nuc_counter(dna:String) -> HashMap<char, usize>{

    let mut new_map:HashMap<char, usize> = HashMap::new();

    for nuc in dna.chars(){
        
        let count = new_map.entry(nuc).or_insert(0); // checks if entry exists or if it doesnt inserts it with a starting value (ie.0)
        *count +=1; //Defines how much the value is increased, accessing the value by dereferencing the mutable reference to get the actual value stored in the map.
    }

    new_map
}

pub fn calculator(a:f64,b:f64,sign:&str){

    match sign {
        "+"=>print!("Result: {}",a+b),
        "-"=>print!("Result: {}",a-b),
        "/"=>print!("Result: {}",a/b),
        "*"=>print!("Result: {}",a*b),
        _=>panic!("Operation to be implemented")
    }

}