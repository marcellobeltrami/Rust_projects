use std::io;

pub fn display_menu() -> i8{

    println!("Select operation number:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Exit");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap(); // here I am effectively mutating choice value

    return choice.trim().parse::<i8>().unwrap();
}


pub fn getnumber() -> (f64,f64){
    println!("Enter first number: ");
    let mut num1_in = String::new();
    io::stdin().read_line(&mut num1_in).unwrap();

    let num_1: f64 = num1_in.trim().parse().expect("Invalid input!");

    println!("Enter second number: ");
    let mut num2_in = String::new();
    io::stdin().read_line(&mut num2_in).unwrap();

    let num_2: f64 = num2_in.trim().parse().expect("Invalid input!");

    return (num_1,num_2); // Returns a touple <immutable list> of values.
}

pub fn calculation(val_1:f64,val_2:f64, choice_operation:i8) -> f64{

    let result = match choice_operation{
        1 => val_1 + val_2,
        2 =>val_1 - val_2,
        3 => val_1 * val_2,
        4 => val_1/val_2,
        _ => panic!("Choice not valid!")
        
    };

    result

}




// Tests for calculation function

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        let result = calculation(2.0, 2.0, 1);
        assert_eq!(result, 4.0);
    }
    #[test]
    fn subtract() {
        let result = calculation(2.0, 2.0, 2);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn multiply() {
        let result = calculation(3.0, 3.0, 3);
        assert_eq!(result, 9.0);
    }

    #[test]
    fn divide() {
        let result = calculation(3.0, 3.0, 4);
        assert_eq!(result, 1.0);
    }

}

