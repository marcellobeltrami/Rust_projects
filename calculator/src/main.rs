use dep::utilities::{getnumber, display_menu,calculation};
mod dep;

fn main() {
    let mut oper_choice: i8 = display_menu();
    
    if oper_choice == 5{
        println!("Exiting...");
        panic!();
    }

    else {
        while  oper_choice != 5{
            let (num_1, num_2 )= getnumber();
            println!("Result: {}",calculation(num_1, num_2, oper_choice));

            oper_choice = display_menu();
        }
        
        println!("Exiting...")
    }

}
