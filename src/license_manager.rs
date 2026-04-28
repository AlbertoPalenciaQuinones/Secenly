use crate::{director::Director, builder::Builder, license_builder::LicenseBuilder, license_asn1::LicenseAsn1};

use std::io::{self, Write};
use rusqlite::{Connection, Result};
use chrono::{DateTime, Duration, Utc};

pub fn connect_db() -> Result<Connection> {
    let conn = Connection::open("secenly.db")?;  
    Ok(conn)
}

pub fn create_license() -> Result<()> {
    // Variable para almacenar el licenseId generado desde la biblioteca
    let id: [u8; 64] = [
        0xdd, 0xaf, 0x35, 0xa1, 0x93, 0x61, 0x7a, 0xba,
        0xcc, 0x41, 0x73, 0x49, 0xae, 0x20, 0x41, 0x31,
        0x12, 0xe6, 0xfa, 0x4e, 0x89, 0xa9, 0x7e, 0xa2,
        0x0a, 0x9e, 0xee, 0xe6, 0x4b, 0x55, 0xd3, 0x9a,
        0x21, 0x92, 0x99, 0x2a, 0x27, 0x4f, 0xc1, 0xa8,
        0x36, 0xba, 0x3c, 0x23, 0xa3, 0xfe, 0xeb, 0xbd,
        0x45, 0x4d, 0x44, 0x23, 0x64, 0x3c, 0xe8, 0x0e,
        0x2a, 0x9a, 0xc9, 0x4f, 0xa5, 0x4c, 0xa4, 0x9f,
    ];
    let notes = String::from("Licencia creada desde CLI");
    
    let duration = loop {
        println!("Select the duration of the license:\n1) 1 day\n2) 7 days\n3) 1 month\n4) 3 months\n5) 6 months\n6) 1 year"
        );
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

        let duration = match option {
            1 => Duration::days(1),
            2 => Duration::days(7),
            3 => Duration::days(30),
            4 => Duration::days(30 * 3),
            5 => Duration::days(30 * 6),
            6 => Duration::days(365),
            _ => {
                println!("Invalid option. Please select a number between 1 and 6.");
                continue;
            }
        };

        break duration;
    };
    let expiration: DateTime<Utc> = Utc::now() + duration;
    
    let heartbeat = loop {
        println!("Select heartbeat interval:\n1) 15 seconds\n2) 30 seconds\n3) 1 minute\n4) 5 minutes\n5) 15 minutes\n6) 1 hour"
        );
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

        let heartbeat = match option {
            1 => 15,
            2 => 30,
            3 => 60,
            4 => 300,
            5 => 900,
            6 => 3600,
            _ => {
                println!("Invalid option. Please select a number between 1 and 6.");
                continue;
            }
        };

        break heartbeat;
    };

    let mut license_builder = LicenseBuilder::default();
    Director::construct_license(&mut license_builder, id, expiration, heartbeat, notes);
    let license = license_builder.build();

    

    Ok(())
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