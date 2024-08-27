mod decrypt;
mod encrypt;
use crate::decrypt::decrypt_directory_recursive;
use crate::encrypt::encrypt_directory_recursive;
use std::env;
use std::fs::File;
use colored::Colorize;

fn main() {
    println!("{}\n{}\n{}", "welcome to cats".bright_purple(), "the world's only mildly acessible way to 'encrypt' files on your computer".green(), "now with Vaults!".yellow());
    println!();

    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();
    let mode: &str = &args[1];
    if mode == "-s" || mode == "--standard" {
        let eord: &str = &args[2];

        let password: &str = &args[3];
        let directory: &str = &args[4];

        if eord == "-e" {
            // Encrypt directory
            if let Err(err) = encrypt_directory_recursive(directory, password) {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
            println!(
                "Encryption of\ndirectory: [{}]\npasssword: [{}]\nresult: sucessful",
                directory, password
            );
        } else if eord == "-d" {
            //  Decrypt directory
            if let Err(err) = decrypt_directory_recursive(directory, password) {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
            println!(
                "Decryption of\ndirectory: [{}]\npasssword: [{}]\nresult: sucessful",
                directory, password
            );
        }
    }
    else if mode == "-v" || mode == "--vaults" {
        let mkvault: &str = &args[2];
        if mkvault == "-mkv" || mkvault == "--makevault" {
            let newvaultpath: &str = &args[3];
            let newvaultpassword: &str = &args[4];
        }
    }
    else {
        eprintln!("{}", "ohhnoooo".red())
    }
}
