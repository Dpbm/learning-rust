use std::io;
use std::process;

fn main() {
    let mut input = String::new();
    
    println!("Type something Dude: ");
    
    match io::stdin().read_line(&mut input){
        Ok(_) => input = input.trim().to_string(),
        Err(error) => {
            println!("Error getting input string!!\n{}", error);
            process::exit(1);
        }
    }
    
    println!("\nBro, You've just typed: {}", input);
   
    match input == String::from("Hello, world!") {
        true => println!("Hello, world! to you too!"),
        false => println!("Nice typing")
    } 
    
}
