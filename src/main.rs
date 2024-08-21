// Bringing the ENV Module. 
use std::env; 
use std::fs;


// Main
fn main() {

    // Simple Command Line Arguments 
    let args: Vec<String> = env::args().collect();     


    // First Two Arguments 
    // Query is the String We Want 
    // Filename, is the File We Want To Search 
    let query = &args[1];
    let filename= &args[2]; 

    
    // Printing to Console.
    println!("Searching For {}", query);
    println!("In File {}", filename);

    // Reads Contents 
    // Else, returns Exception 
    let contents = fs::read_to_string(filename).expect("Something went wrong!");


    println!("With text \n{}", contents);


}