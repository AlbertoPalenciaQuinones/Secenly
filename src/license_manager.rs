use crate::{builder::Builder, director::Director, encapsulate::EncapsulateService, license_builder::LicenseBuilder, product_identifier::ProductIdentifier, license_id_generator::LicenseIdGenerator};

use std::io::{self, Write};
use chrono::{Duration, Timelike, Utc};
use std::process::Command;
use std::path::PathBuf;

use rusqlite::{Result};
//use rusqlite::{Connection};

// Comandos:
// openssl asn1parse -inform DER -in licenses/license.der  ->  comprueba formato ASN.1
// hexdump -C licenses/license.der  ->  muestra el contenido del archivo DER en formato hexadecimal

/*pub fn connect_db() -> Result<Connection> {
    let conn = Connection::open("secenly.db")?;  
    Ok(conn)
}*/

pub fn create_license() -> Result<()> {
    let path = ask_path();

    let product_identifier = ProductIdentifier::new(path, obtain_hwid());
    let license_id = LicenseIdGenerator::generate(product_identifier.get_product_id().as_str());
    print!("Generated License ID: {}\n", license_id);

    let duration = ask_option(
        "Select the duration of the license:\n\
         1) 1 day\n2) 7 days\n3) 1 month\n4) 3 months\n5) 6 months\n6) 1 year",
        |option| match option {
            1 => Some(Duration::days(1)),
            2 => Some(Duration::days(7)),
            3 => Some(Duration::days(30)),
            4 => Some(Duration::days(90)),
            5 => Some(Duration::days(180)),
            6 => Some(Duration::days(365)),
            _ => None,
        },
    );

    let expiration = Utc::now()
        .with_nanosecond(0)
        .unwrap()
        + duration;

    let heartbeat = ask_option(
        "Select heartbeat interval:\n\
         1) 15 seconds\n2) 30 seconds\n3) 1 minute\n4) 5 minutes\n5) 15 minutes\n6) 1 hour",
        |option| match option {
            1 => Some(15),
            2 => Some(30),
            3 => Some(60),
            4 => Some(300),
            5 => Some(900),
            6 => Some(3600),
            _ => None,
        },
    );

    let notes = String::from("Licencia creada desde CLI");

    let mut license_builder = LicenseBuilder::default();

    Director::construct_license(&mut license_builder, license_id, expiration, heartbeat, notes);
    let license = license_builder.build();

    EncapsulateService::encapsulate_license(license);

    Ok(())
}

fn ask_path() -> PathBuf {
    loop {
        println!("Escriba la ruta donde se guardará la semilla:");
        print!("Ruta: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input, try again.");
            continue;
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            println!("La ruta no puede estar vacía.");
            continue;
        }

        return PathBuf::from(trimmed);
    }
}

fn ask_option<T>(prompt: &str, parser: impl Fn(u32) -> Option<T>) -> T {
    loop {
        println!("{}", prompt);
        print!("Option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input, try again.");
            continue;
        }

        let option: u32 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Please, enter a valid number.");
                continue;
            }
        };

        if let Some(result) = parser(option) {
            return result;
        }

        println!("Invalid option.");
    }
}

pub fn obtain_hwid() -> String {
    let output = if cfg!(target_os = "linux") {
        Command::new("cat")
            .arg("/etc/machine-id")
            .output()
            .expect("Error ejecutando comando")
    } else if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args([
                "-Command",
                "(Get-CimInstance Win32_ComputerSystemProduct).UUID",
            ])
            .output()
            .expect("Error ejecutando comando en Windows")
    } else {
        panic!("SO no soportado");
    };

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string()
}

pub fn delete_license() -> Result<()> {
    println!("Seleccione la licencia que desea eliminar");
    // MOSTRAR LAS LICENCIAS ASOCIADAS A ESE USUARIO

    Ok(())
}

pub fn modify_license() -> Result<()> {
    println!("Seleccione la licencia que desea modificar");
    // MOSTRAR LAS LICENCIAS DE LA BBDD CUYO HWID CORRESPONDA AL DEL USUARIO
    // Mas bien, muestra los softwares que cuentan con licencia el usuario

    Ok(())
}

pub fn read_license() -> Result<()> {
    //let data = read("license.lic").unwrap();
    //println!("RAW DER bytes: {:02X?}", data);
    //let license: License = rasn::der::decode(&data).unwrap();
    Ok(())
}

pub fn list_licenses() -> Result<()> {
    println!("Licencias arraigadas a su usuario:");
    // MOSTRAR LAS LICENCIAS DE LA BBDD CUYO HWID CORRESPONDA AL DEL USUARIO
    Ok(())
}