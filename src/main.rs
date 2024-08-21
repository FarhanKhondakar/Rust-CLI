// Bringing the ENV Module. 
use std::env; 
use std::fs;
use std::process;


// Main
fn main() {

    // Simple Command Line Arguments 
    let args: Vec<String> = env::args().collect();     

    // Getting The Args from Config Implementation 
    // If OK, then unpackages/deconstructs
    // Else, it exits/quits the program
    let config = Config::new(&args).unwrap_or_else(|err| { 
        println!("Problem Parsing Arguments: {}", err); 
        process::exit(1);
    });

    
    // Printing to Console.
    println!("Searching For {}", config.query);
    println!("In File {}", config.filename);

    // Reads Contents 
    // Else, returns Exception 
    let contents = fs::read_to_string(config.filename).expect("Something went wrong!");


    println!("With text \n{}", contents);


}



struct Config { 
    query: String, 
    filename: String, 
}

impl Config { 
    // Function is a "new" Config, based on Implementation of Config
    fn new(args: &[String]) -> Result<Config,&str> { 
        // Safety Check 

        if args.len() < 3 {
           return Err("not enough arguments");
        }


        // Query & Filename Arguments 
        let query = args[1].clone();
        let filename= args[2].clone(); 
    
        // Return Tuple
        Ok(Config { query, filename })
    
    }
}
