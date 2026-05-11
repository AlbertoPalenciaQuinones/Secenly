use std::io::{self, Write};
use std::path::PathBuf;

// Pide que se introduzca la ruta de algunos archivos o ficheros.
pub fn ask_path(file: &str) -> PathBuf {
    let mut folder;
    loop {
        println!("[INPUT] Write the folder where the {}: ", file);
        io::stdout().flush().unwrap();

        folder = String::new();

        if io::stdin().read_line(&mut folder).is_err() {
            println!("[ERROR] Reading input, try again.");
            continue;
        }

        let trimmed = folder.trim();

        if trimmed.is_empty() {
            println!("[ERROR] The path can't be empty, try again.");
            continue;
        }

        return PathBuf::from(trimmed)
    }
}

// Recibe la opción de algunos parámetros de la licencia.
pub fn ask_option<T>(prompt: &str, parser: impl Fn(u32) -> Option<T>) -> T {
    loop {
        println!("{}", prompt);
        print!("Option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            println!("[ERROR] Reading input, try again.");
            continue;
        }

        let option: u32 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("[ERROR] Invalid number, try again.");
                continue;
            }
        };

        if let Some(result) = parser(option) {
            return result;
        }

        println!("[ERROR] Invalid option, try again.");
    }
}
