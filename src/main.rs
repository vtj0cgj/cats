mod decrypt;
mod encrypt;
use crate::decrypt::decrypt_directory_recursive;
use crate::encrypt::encrypt_directory_recursive;
use colored::Colorize;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::error::Error;

fn modifyvault(dir: &str, password: &str) -> std::io::Result<()> {
    let mut file: File = File::create(dir)?;
    let mut encrypted_data: String = String::new();
    file.read_to_string(&mut encrypted_data);
    let decrypted_data = decrypt::decrypt(&encrypted_data, password, false);
    Ok(())
    // unfinushed function, not implemented.
}

fn openvault(dir: &str, password: &str) -> std::io::Result<()> {
    let mut file: File = File::create(dir)?;
    let mut encrypted_data: String = String::new();
    file.read_to_string(&mut encrypted_data);
    let decrypted_data: Result<Vec<u8>, Box<dyn Error>> = decrypt::decrypt(&encrypted_data, password, false);
    println!("{:?}", decrypted_data);
    Ok(())
    // not implemented
}

fn makevault(dir: &str, password: &str) -> std::io::Result<()> {
    let mut file = File::create(dir)?;
    let header: &str = "CATS v0.0:DEV\n";
    file.write_all(header.as_bytes())?;

    Ok(())
}

fn main() {
    println!("{}\n{}\n{}\n{}", "cats".bright_purple(), "v0.0:DEV", "the world's only mildly acessible way to 'encrypt' files on your computer".green(), "now with Vaults!".yellow());
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
            }
            println!(
                "Decryption of\ndirectory: [{}]\npasssword: [{}]\nresult: sucessful",
                directory, password
            );
        }
    }
    else if mode == "-v" || mode == "--vaults" {
        let whtvault: &str = &args[2];
        if whtvault == "-mkv" || whtvault == "--makevault" {
            let newvaultpath: &str = &args[3];
            let newvaultpassword: &str = &args[4];
            let _ = makevault(newvaultpath, newvaultpassword);
        }
        else if whtvault == "-rd" || whtvault == "--readvault" {
            let vaultpath: &str = &args[3];
            let vaultpassword: &str = &args[4];
            let mut input_file: File = File::open(vaultpath).expect("L: failed to open vault");
            let mut encrypted_data: String = String::new();
            let _ = input_file.read_to_string(&mut encrypted_data);
            let decrypted_data = decrypt::decrypt(&encrypted_data, vaultpassword, false);
            println!("{:?}", decrypted_data);
        }
        else if whtvault == "-md" || whtvault == "--modifyvault" {
            let vaultpath: &str = &args[3];
            let vaultpassword: &str = &args[4];
        }
    }
    else {
        eprintln!("{}\n{}", "Invalid arguments:".red(), "syntax:\nR")
    }
}
