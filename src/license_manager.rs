use std::fs::{read};
use crate::{director::Director, license::License};
use crate::{builder::Builder, license_builder::LicenseBuilder};
//use rasn::{AsnType, Encode, Decode};
use rusqlite::{Connection, Result};

pub fn connect_db() -> Result<Connection> {
    let conn = Connection::open("secenly.db")?;  
    Ok(conn)
}

pub fn create_license() -> Result<()> {
    print!("Proceso de creación de licencia\n");

    let mut license_builder = LicenseBuilder::default();
    Director::construct_license(&mut license_builder, [0; 64]);
    let license = license_builder.build();

    println!("{:?}", license);
    //println!("License built: {:?}\n", license.get_id());
    //println!(" {:?}\n", license.get_creation_date());
    //println!(" {:?}\n", license.get_expiration_date());
    //println!(" {:?}\n", license.get_last_use_date());
    //println!(" {:?}\n", license.get_heartbeat_interval());
    //println!(" {:?}\n", license.get_notes());

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
    let data = read("license.lic").unwrap();
    println!("RAW DER bytes: {:02X?}", data);
    let license: License = rasn::der::decode(&data).unwrap();
    Ok(())
}

pub fn list_licenses() -> Result<()> {
    println!("Licencias arraigadas a su usuario:");
    // MOSTRAR LAS LICENCIAS DE LA BBDD CUYO HWID CORRESPONDA AL DEL USUARIO
    Ok(())
}