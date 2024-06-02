mod decrypt;
mod encrypt;
use std::env;

fn main() {
    println!("welcome to cats, which is an acronym for something i forgot.");
    println!();
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();
    let eord: &str = &args[1];

    let password: &str = &args[2];
    let directory: &str = &args[3];

    if eord == "-e" {
        // Encrypt directory
        if let Err(err) = encrypt_directory_recursive(directory, password) {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
    else if eord == "-d" {
        //  Decrypt directory
        if let Err(err) = decrypt_directory_recursive(directory, password) {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }

    
}
