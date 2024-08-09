// Enemy/Boss implement fn such as attack, charge. 
use std::str::FromStr;
use std::collections::HashMap;
extern crate rand;
use rand::seq::IteratorRandom;


//Player 
pub struct Player{
    pub name: String,
    pub health: i16,
    pub description : String,
    pub electricity: i16,
    pub archetype: String,
    pub attacks: HashMap<String, i32>,
    pub cybernetics: HashMap<String, i32>,
    pub inventory: HashMap<String, (i32,i32)>,  

}


// Allows code to be extended including multiple archetype
impl Player 
    {
        
        // Gunner archetype
        pub fn mercer_gunner()-> Player{
            let mut mercer_gun = Player{
            name: String::from_str("Mercer, Alex").expect("Name Error"),
            health: 120,
            description: String::from_str("A former law enforcer turned rogue, equipped with a high-tech Malvolant mk.3 revolver and advanced cybernetic enhancements.").expect("Usr Desc erro"),
            archetype: String::from_str("Gunner").expect("Arch Error"),
            electricity: 80,
            attacks: HashMap::new(),
            cybernetics: HashMap::new(),
            inventory: HashMap::new()};

        // Mercer gunner abilities
            mercer_gun.attacks.insert(String::from("CS"), 20); // Cyber Shot. 
            mercer_gun.attacks.insert(String::from("NS"), 30 ); // Nano Shield
            mercer_gun.attacks.insert(String::from("HD"), 40 ); // Holo Decoy
            mercer_gun.attacks.insert(String::from("DS"), 1); // Dark shot
            
            return  mercer_gun;
        }

        // Mercer samurai archetype
        pub fn mercer_samurai()-> Player{
            let mut mercer_blades = Player{
            name: String::from_str("Mercer, Alex").expect("Name Error"),
            health: 130,
            archetype: String::from_str("Gunner").expect("Arch Error"),
            description: String::from_str("A disciplined warrior with both physical prowess and enhanced reflexes, wielding twin energy blades.").expect("Usr Desc erro"),
            electricity: 70,
            attacks: HashMap::new(),
            cybernetics: HashMap::new(),
            inventory: HashMap::new()};

            // Mercer samurai abilities
            mercer_blades.attacks.insert(String::from("BF"), 15); // Blade Flurry 
            mercer_blades.attacks.insert(String::from("DF"), 30  ); // Deflect 
            mercer_blades.attacks.insert(String::from("AB"), 40 ); // Adrenaline Boost
            mercer_blades.attacks.insert(String::from("SLS"), 1); // Swan Last Song
                
            return  mercer_blades;
        }

}

//Enemy: implement different enemy types
pub struct Enemy{
    pub name: String,
    pub health: i16,
    pub description: String, //include weakness here
    pub attacks: HashMap<String, i32>,
    pub loot: HashMap<String, (i32,i32)>,  // name of item and its stats
}



impl Enemy {
    
    pub fn loot_randomizer() -> HashMap<String, (i32,i32)> {
        // Defines items and to store vector.
        let mut items: HashMap<String, (i32,i32)> = HashMap::new();
        
        // Example item Add more items.
        items.insert("Health Nanites".to_string(), (1,20)); // quantity, RPG value

        // Randomly picks an item and returns it with its stats.
        
            // Randomly pick one item from items and returns as Hashmap
            let mut rng = rand::thread_rng();
            if let Some((name, &(stat1,stat2))) = items.iter().choose(&mut rng) {
                let mut loot = HashMap::new();
                loot.insert(name.clone(), (stat1, stat2));
                return loot;
            } 
        
        // Returns empty array if item is not found (make borrow-checker happy)
        return HashMap::new(); 
            
    }

    

    // Enemy corpo enforcer
    pub fn corpo_enfo()->Enemy{
        
        
        let mut corpo_enforc = Enemy{

            name: ("NexaGen Enforcer").to_string(),
            health: 150,
            description: ("A heavily armored soldier employed by mega-corporations to maintain order and eliminate threats.").to_string(), //include weakness here
            attacks: HashMap::new(),
            loot: Self::loot_randomizer(), 
        };
        

        corpo_enforc.attacks.insert(("Suppressive Fire").to_string(), 20); 
        corpo_enforc.attacks.insert("Nano Repair".to_string(), 30);
        corpo_enforc.attacks.insert("Shock Grenade:".to_string(), 40);



        corpo_enforc

    }

}
