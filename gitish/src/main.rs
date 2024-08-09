mod dep;
use clap::{Command, Arg};
use dep::versioning::Repo;

fn main() {
    
    let matches = Command::new("gitish")
    .version("1.0")
    .author("Marcello Beltrami")
    .about("git-like application in rust.")
     //Sub-command init 
    .subcommand( 
        Command::new("init")
            .about("intializes repo to selected directory")
            .arg(Arg::new("curr_repo").help("defines repo to be initiliazed").required(true).index(1))
    )
     
     //Sub-command clone added here 
    .subcommand(   
        Command::new("clone")
        .about("main app command")
        .arg(Arg::new("url").help("The item to add").required(true).index(1))
        .arg(Arg::new("usr_repo").help("The item to add").required(true).index(2)), 
    )
   
    
    .get_matches();

    // Checks commands against user input and executes relative piece of code. 
    match matches.subcommand(){
        Some(("init", sub_m)) => {
            let curr_repp = sub_m.get_one::<String>("curr_repo").unwrap();
            
            let repo_st = Repo{usrrepo: curr_repp.clone()};
            repo_st.init();
        }

        Some(("clone", clone)) => {
            let usr_repo = clone.get_one::<String>("usr_repo").unwrap();
            let url_repo = clone.get_one::<String>("url").unwrap();
            let repo_st = Repo{usrrepo: usr_repo.clone()};
            repo_st.clone(url_repo.clone(),);
        }

        _ => {println!("No valid command was used")}


    }

}

