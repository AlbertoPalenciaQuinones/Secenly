mod builder;
mod domain;
mod services;
mod application;
mod input;

use crate::domain::license_error::LicenseError;

fn main() -> Result<(), LicenseError> {
    println!("\n\n\n\n=====================================================================");
    println!("======================== WELCOME TO SECENLY! ========================");
    println!("=====================================================================\n");
    application::run()?;
    Ok(())
}


