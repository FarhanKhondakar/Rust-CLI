// Bringing the ENV Module. 
use std::env; 
use std::fs;


// Main
fn main() {

    // Simple Command Line Arguments 
    let args: Vec<String> = env::args().collect();     

    // Deconstructing the Tuple -> Query & Filename
    let (query, filename) = parse_configs(&args);

    
    // Printing to Console.
    println!("Searching For {}", query);
    println!("In File {}", filename);

    // Reads Contents 
    // Else, returns Exception 
    let contents = fs::read_to_string(filename).expect("Something went wrong!");


    println!("With text \n{}", contents);


}

fn parse_configs(args: &[String]) -> (&str, &str) { 

    // Query & Filename Arguments 
    let query = &args[1];
    let filename= &args[2]; 

    // Return Tuple
    (query, filename)

}