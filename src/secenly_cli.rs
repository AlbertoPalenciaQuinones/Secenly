use crate::license_manager::{create_license, delete_license, modify_license, list_licenses, read_license};

use std::env;
use std::io::{self, Write};

// GUÍA DE ESTILO
//https://doc.rust-lang.org/stable/style-guide/items.html

fn show_menu() {
    println!("=====================================================================");
    println!("======================== WELCOME TO SECENLY! ========================");
    println!("=====================================================================\n");
    println!("Select an option:");
    println!("1. Generate a new license");
    println!("2. Delete an existing license");
    println!("3. Modify a license");
    println!("4. List all licenses");
    println!("5. Read an existing license from an archive");
    println!("0. Exit");
}

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let opt = if args.len() > 1 {
        args[1].trim().to_string()
    } else {
        show_menu();
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };

    match opt.as_str() {
        "1" | "create" => {
            if let Err(e) = create_license() {
                eprintln!("Error: {}", e);
            }
        }
        "2" | "delete" => {
            if let Err(e) = delete_license() {
                eprintln!("Error: {}", e);
            }
        }
        "3" | "modify" => {
            if let Err(e) = modify_license() {
                eprintln!("Error: {}", e);
            }
        }
        "4" | "list" => {
            if let Err(e) = list_licenses() {
                eprintln!("Error: {}", e);
            }
        }
        "5" | "read" => {
            if let Err(e) = read_license() {
                eprintln!("Error: {}", e);
            }
        }
        "0" | "exit" => println!("Saliendo del programa"),
        _ => println!("Opción no válida"),
    }
}