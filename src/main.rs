// Bringing the ENV Module. 
use std::env; 
use std::fs;
use std::string;


// Main
fn main() {

    // Simple Command Line Arguments 
    let args: Vec<String> = env::args().collect();     

    // Deconstructing the Tuple -> Query & Filename
    let config = parse_configs(&args);

    
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

fn parse_configs(args: &[String]) -> Config { 

    // Query & Filename Arguments 
    let query = args[1].clone();
    let filename= args[2].clone(); 

    // Return Tuple
    Config { query, filename }

}