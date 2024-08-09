use git2::Repository;


// defines repo utilities
pub struct Repo{
    
    pub usrrepo:String
}

// implements repo functionalities. Returns the Repository struc.
impl Repo {

    pub fn init(&self) -> Repository{
        println!("Initializing repo at {}...", self.usrrepo);
        
        let repo = match Repository::init(&self.usrrepo) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to init: {}", e),
        };

        return repo;
        
    }
    
    pub fn clone(&self, githuburl:String)-> Repository{

        println!("Cloning {} into {}...", githuburl,self.usrrepo);

        let repo = match Repository::clone(&githuburl, &self.usrrepo) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
            
        };

        return repo;

    }

    pub fn commit(&self){
        println!("Merging and updating changes in {}", &self.usrrepo);


        let repo = match Repository::open(&self.usrrepo) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };




    }



}

