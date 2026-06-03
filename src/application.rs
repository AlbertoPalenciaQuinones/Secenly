use std::env;
use std::fs::File;
use std::io::{self, Write};

use crate::exceptions::LicenseError;
use crate::builder::{builder::Builder, director::Director, license_builder::LicenseBuilder};      
use crate::services::{encapsulate::EncapsulateService, hardware_manager::HardwareManager,
            license_manager::LicenseManager, product_manager::ProductManager};

use crate::input::{ask_i32, ask_option, ask_path, ask_string};

use chrono::{DateTime, Duration, NaiveDate, Timelike, Utc};
use rusqlite::{Connection, Result};

// Mensaje inicial de Secenly
fn show_menu() {
    println!("Select an option:");
    println!("1. Generate a new license");
    println!("2. Delete an existing license");
    println!("3. Modify a license");
    println!("4. Read licenses assigned to the hardware");
    println!("5. Obtain license as a file");
    println!("6. Obtain seed as a file");
    println!("0. Exit");
}

// Función principal del programa. Recibe por línea de comandos la función a
// realizar en la herramienta (0-4)
pub fn run() -> Result<(), LicenseError> {
    let conn = connect_db()?;
    loop {
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
                if let Err(e) = create_license(&conn) {
                    eprintln!("Error: {}", e);
                }
            }
            "2" | "delete" => {
                if let Err(e) = delete_license(&conn) {
                    eprintln!("Error: {}", e);
                }
            }
            "3" | "modify" => {
                if let Err(e) = modify_license(&conn) {
                    eprintln!("Error: {}", e);
                }
            }
            "4" | "read" => {
                let hw = HardwareManager::new()?;
                if let Err(e) = read_licenses(&conn, &hw.get_hardware_id()) {
                    eprintln!("Error: {}", e);
                }
            }
            "5" | "obtain_license" => {
                if let Err(e) = obtain_license_file(&conn) {
                    eprintln!("Error: {}", e);
                }
            }
            "6" | "obtain_seed" => {
                if let Err(e) = obtain_seed_file(&conn) {
                    eprintln!("Error: {}", e);
                }
            }
            "0" | "exit" => {
                println!("Closing Secenly...");
                break Ok(());
            }
            _ => {
                println!("Invalid option, try again.");
                continue; 
            }
        }

        // Evita que, si pasaron argumentos por CLI, se repita en bucle
        if args.len() > 1 {
            break Ok(());
        }
    }
}

fn connect_db() -> Result<Connection, LicenseError> {
    let conn = Connection::open("secenly.db")?;  
    Ok(conn)
}

// OPCIÓN 1: Crear una nueva licencia y almacenarla en la base de datos
fn create_license(conn: &Connection) -> Result<(), LicenseError> {
    // Obtención de rutas para almacenar la semilla, licencia y donde se
    // encuentra el certificado y la clave privada.
    let license_path = ask_path(
        &"license will be save (with his name and .der extension)".to_string());
    let seed_path = ask_path(
        &"seed is locate or where you want to save (with his name)".to_string());
    let cert_path = ask_path(
        &"certificate is locate (with his name)".to_string());
    let key_path = ask_path(
        &"private key is locate (with his name)".to_string());

    // Creación de los managers necesarios para generar la licencia
    let hardware_manager = HardwareManager::new()?;
    let product_manager = ProductManager::new(
        &seed_path, hardware_manager.get_hardware_id())?;
    let license_manager = LicenseManager::new(
        &product_manager.get_product_id())?;

    // Obtención de la duración de la licencia
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

    // Obtención de la expiración de la licencia
    let expiration = Utc::now()
        .with_nanosecond(0)
        .unwrap()
        + duration;

    // Obtención del intervalo de latidos
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
    
    let notes = String::from("License generated by CLI");

    let mut license_builder = LicenseBuilder::default();

    // Construcción de la licencia
    Director::construct_license(&mut license_builder, 
        license_manager.get_license_id().to_string(), 
        expiration, 
        heartbeat, 
        notes
    );

    let license = license_builder.build();

    // Obtención de la licencia codificada en DER
    let der_bytes = EncapsulateService::encapsulate_license(
        license, 
        &license_path, 
        &cert_path, 
        &key_path
    )?;

    // Almacenarla en la base de datos
    conn.execute(
        "INSERT INTO licenses (hwid, seed, license_der)
         VALUES (?1, ?2, ?3)",
        rusqlite::params![hardware_manager.get_hardware_id(), 
                          product_manager.get_seed(), 
                          der_bytes],)?;
    
    Ok(())
}

// OPCIÓN 2: Eliminar una licencia (de la base de datos)
fn delete_license(conn: &Connection) -> Result<(), LicenseError> {
    // Obtener el identificador de licencia a eliminar
    let (id, _) = choose_license(conn, None)?;

    // Eliminarla de la base de datos
    conn.execute("DELETE FROM licenses WHERE id = ?1", [id])?;

    println!("[INFO] License deleted");

    Ok(())
}

// OPCIÓN 3: Modificar una licencia y actualizar en la base de datos
fn modify_license(conn: &Connection) -> Result<(), LicenseError> {
    // Obtener el identificar y bytes de la licencia a modificar
    let (id, der) = choose_license(conn, None)?;

    // Obtener los campos de licencia deserializándola
    let mut license = EncapsulateService::decapsulate_license(&der)?;

    println!("1. Expiration date");
    println!("2. Heartbeat");
    println!("3. Notes");

    let option = select_index(3, "Select field:");

    // Dependiendo de la opción introducida, se modifican distintos parámetros
    match option {
        // Fecha de expiración
        0 => {
            loop {
                let input = ask_string("New expiration (DD/MM/YYYY): ");

                // Parseo la fecha (formato DD/MM/YYYY)
                let date = match NaiveDate::parse_from_str(&input, "%d/%m/%Y") {
                    Ok(d) => d,
                    Err(_) => {
                        println!("[ERROR] Invalid format. Use DD/MM/YYYY");
                        continue;
                    }
                };

                let today = Utc::now().date_naive();

                // Fecha máxima de expiración
                let max_date = today
                    .checked_add_days(chrono::Days::new(365))
                    .unwrap(); // seguro aquí

                // Validación 1: fecha pasada o igual a hoy
                if date <= today {
                    println!("[ERROR] Date must be greater than today");
                    continue;
                }

                // Validación 2: fecha mayor que la fecha máxima
                if date > max_date {
                    println!("[ERROR] Date cannot exceed 1 year from today");
                    continue;
                }

                license.expiration_date = DateTime::<Utc>::from_naive_utc_and_offset(
                    date.and_hms_opt(0, 0, 0).unwrap(),
                    Utc,
                );

                break;
            }
        }
        // Intervalo del latido
        1 => {
            loop {
                let value = ask_i32("New heartbeat (seconds): ");

                // Validación 1: valor menor o igual a 0
                if value <= 0 {
                    println!("[ERROR] Heartbeat must be greater than 0");
                    continue;
                }

                // Validación 2: valor mayor a 3600 segundos
                if value > 3600 {
                    println!("[ERROR] Heartbeat cannot exceed 3600 seconds");
                    continue;
                }

                license.heartbeat_interval = value;
                break;
            }
        }
        // Notas de la licencia
        2 => {
            license.notes = ask_string("New notes: ");
        }
        _ => unreachable!(),
    }

    let license_path = ask_path(
        &"license will be save (with his name and .der extension)".to_string());

    // Volver a serializar la licencia
    let new_der = EncapsulateService::encapsulate_license(
        license,
        &license_path,
        &ask_path(&"certificate is locate (with his name)"),
        &ask_path(&"private key is locate (with his name)"),
    )?;

    // Almacenarla en la base de datos
    conn.execute(
        "UPDATE licenses SET license_der = ?1 WHERE id = ?2",
        rusqlite::params![new_der, id],
    )?;

    println!("[INFO] License updated");

    Ok(())
}

// OPCIÓN 4: Leer licencias atadas al hardware almacenadas en la base de datos
fn read_licenses(conn: &Connection, hwid: &String) -> Result<(), LicenseError> {
    // Obtener las licencias
    let licenses = obtain_licenses(conn, Some(hwid))?;

    ensure_not_empty(&licenses)?;

    print_licenses(&licenses);

    Ok(())
}

// OPCIÓN 5: Obtener licencia como un archivo
fn obtain_license_file(conn: &Connection) -> Result<(), LicenseError> { 
    // Obtener la licencia
    let (_, der) = choose_license(conn, None)?;

    // Obtener la ruta donde se almacenará la licencia
    let license_path = ask_path(
        &"license will be save (with his name and .der extension)".to_string());

    // Crear el archivo y escribir la licencia
    let mut file = File::create(&license_path)
        .map_err(|e| LicenseError::WriteWithContext {
            source: e,
            path: license_path.display().to_string(),
        })?;

    file.write_all(&der)
        .map_err(|e| LicenseError::WriteWithContext {
            source: e,
            path: license_path.display().to_string(),
        })?;

    println!("[INFO] License saved at {:?}", license_path);

    Ok(())
}

// OPCIÓN 6: Obtener la semilla como un archivo
fn obtain_seed_file(conn: &Connection) -> Result<(), LicenseError> {
    // Obtener la licencia 
    let (id, _) = choose_license(conn, None)?;

    // Obtener la ruta donde se almacenará la semilla
    let seed_path = ask_path(
        &"where the seed will be saved (with his name and .dat extension)".to_string());

    // Obtener la semilla de la BBDD
    let seed: Vec<u8> = conn.query_row(
        "SELECT seed FROM licenses WHERE id = ?1",
        [id],
        |row| row.get(0),
    )?;

    // Crear el archivo y escribir la semilla
    let mut file = File::create(&seed_path)?;
    file.write_all(&seed)?;

    println!("[INFO] Seed saved at {:?}", seed_path);

    Ok(())
}

// Flujo para escoger una licencia de la base de datos (eliminar y modificar)
fn choose_license(conn: &Connection, hwid: Option<&String>
) -> Result<(i32, Vec<u8>), LicenseError> {

    let licenses = obtain_licenses(conn, hwid)?;

    ensure_not_empty(&licenses)?;

    print_licenses(&licenses);

    let index = select_index(licenses.len(), "Choose license:");

    Ok(licenses[index].clone())
}

// Asegurar que no esté vacía la lista de licencias (eliminar, modificar y leer)
fn ensure_not_empty<T>(items: &[T]) -> Result<(), LicenseError> {
    if items.is_empty() {
        return Err(LicenseError::EmptyList);
    }
    Ok(())
}

// Elegir el índice de licencia o parámetro a modificar (eliminar y modificar(x2))
fn select_index(max: usize, message: &str) -> usize {
    ask_option(message, |option| {
        let idx = (option as usize).checked_sub(1)?;
        if idx < max { Some(idx) } else { None }
    })
}

// Obtener licencias de la base de datos (eliminar, modificar y leer)
pub fn obtain_licenses(conn: &Connection, hwid: Option<&String>
) -> Result<Vec<(i32, Vec<u8>)>, LicenseError> {

    let mut stmt = conn.prepare(if hwid.is_some() {
        "SELECT id, license_der FROM licenses WHERE hwid = ?1"
    } else {
        "SELECT id, license_der FROM licenses"
    })?;

    let iter = match hwid {
        Some(h) => stmt.query_map([h], map_row)?,
        None => stmt.query_map([], map_row)?,
    };

    Ok(iter.collect::<Result<_, _>>()?)
}

// Obtener de la base de datos algunos valores de una fila
fn map_row(row: &rusqlite::Row) -> rusqlite::Result<(i32, Vec<u8>)> {
    Ok((row.get(0)?, row.get(1)?))
}

// Imprimir las licencias "crudas" junto a su identificador en la base de datos
fn print_licenses(licenses: &[(i32, Vec<u8>)]) {
    println!("---------------------------------------------------------------------------");
    for (i, (id, der)) in licenses.iter().enumerate() {
        println!("{}. License with {} BBDD ID:", i + 1, id);

        match EncapsulateService::decapsulate_license(der) {
            Ok(license) => {
                println!("ID: {}", license.id);
                println!("Creation date: {}", license.creation_date);
                println!("Expiration date: {}", license.expiration_date);
                println!("Heartbeat: {}", license.heartbeat_interval);
                println!("Notes: {}", license.notes);
            }
            Err(e) => {
                println!("Error reading license: {}", e);
            }
        }

    println!("---------------------------------------------------------------------------");
    }
}