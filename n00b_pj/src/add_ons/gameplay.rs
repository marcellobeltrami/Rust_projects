use rand::seq::SliceRandom;
use std::io::{self, Write};

const SIZE: usize = 4;
const PAIRS: usize = SIZE * SIZE / 2;

pub fn hack_game(msg: String) {
    println!("Entering the {} matrix....", msg);
    
    // Hexadecimal values to be matched
    let hex_values = [
        "0x1", "0x2", "0x3", "0x4", "0x5", "0x6", "0x7", "0x8",
        "0x1", "0x2", "0x3", "0x4", "0x5", "0x6", "0x7", "0x8",
    ];

    let mut grid = hex_values.clone();
    let mut rng = rand::thread_rng();
    grid.shuffle(&mut rng);

    let mut revealed = vec![false; SIZE * SIZE];
    let mut attempts = 0;
    let mut matches = 0;

    while matches < PAIRS {
        print_grid(&grid, &revealed);
        let (first, second) = get_player_choice();

        if first == second || revealed[first] || revealed[second] {
            println!("Invalid selection. Try again.");
            continue;
        }

        attempts += 1;
        revealed[first] = true;
        revealed[second] = true;
        print_grid(&grid, &revealed);

        if grid[first] == grid[second] {
            println!("Match found!");
            matches += 1;
        } else {
            println!("No match. Try again.");
            revealed[first] = false;
            revealed[second] = false;
        }
    }

    println!("Congratulations! You matched all pairs in {} attempts.", attempts);
}

fn print_grid(grid: &[&str], revealed: &[bool]) {
    println!("Current grid:");
    for i in 0..SIZE {
        for j in 0..SIZE {
            let index = i * SIZE + j;
            if revealed[index] {
                print!(" {} ", grid[index]);
            } else {
                print!(" XX ");
            }
        }
        println!();
    }
}

fn get_player_choice() -> (usize, usize) {
    let mut first = String::new();
    let mut second = String::new();

    println!("Enter the position of the first hex value to reveal (0-15): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut first).expect("Failed to read line");
    let mut first: usize = first.trim().parse().expect("Please enter a number");

    if first > 15{
        println!("Value inputted too high, input set to 15.")
    }

    println!("Enter the position of the second hex value to reveal (0-15): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut second).expect("Failed to read line");
    let mut second: usize = second.trim().parse().expect("Please enter a number");
    
    if first > 15{
        println!("Value inputted too high, setting input set to 15.");
        first = 15;
         
    } 
    if second > 15{
        println!("Value inputted too high, setting input to 15.");
        second = 15; 

    }

    (first, second)
}
