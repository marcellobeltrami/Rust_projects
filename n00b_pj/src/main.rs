mod add_ons;
use rand::Rng;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use add_ons::utils;
use add_ons::characters;
use add_ons::gameplay;


fn main() {
    println!("-----------------------------------------------------");
    println!("Booting up...");
    println!("-----------------------------------------------------");
    sleep(Duration::from_secs(2) );
    println!("The year is 2098. In the sprawling metropolis of Midnight City 83, a place where towering skyscrapers pierce the sky and 
    the streets are bathed in the glow of neon lights, the line between humanity and technology has blurred. 
    The city is a melting pot of cybernetic enhancements, powerful megacorporations, and rogue hackers who lurk in the digital shadows.");
    println!("");
    print!("Do you have what it takes to survive? <y/n>... ");
    let _boot_ans = utils::usr_input("");
    
    if _boot_ans.trim() == "n"{
    
        let quotes = vec!["There is nothing like a dream to create the future.",
        "A person can change his future by merely changing his attitude.", 
        "You can't go back and change the beginning, but you can start where you are and change the ending."];
        
        let random_index: usize = rand::thread_rng().gen_range(0..quotes.len());
        
        println!("{:?}",quotes[random_index]);
        exit(0)
    }

    if _boot_ans.trim() == "y"{
        println!("Great, time to forge you an identity");

        let player= characters::Player {
            name: utils::usr_input("--> Name: "),
            main_attack: utils::usr_input("--> main attack name[]: "),
            archetype: utils::usr_input("--> your archetype[]: "), 
            cybernetics: utils::usr_input("--> cybernetics you have[]: "),
            inventory: vec![],
            health: 10,
        };
        //Add code that saves player struc to a json file (see )
        println!("Lets get the journey started{}!", player.name);
    }
    
    println!("As you walk doen the street you see a locked door. Your Kiroshi optic tells you there is something behind. ");
    println!("");
    let _hack_init = utils::usr_input("Would you like to hack the door <y/n>");

    if _hack_init.trim() == "y"{

        gameplay::hack_game("Cybernetic shop door".to_string());

    }


    
}

