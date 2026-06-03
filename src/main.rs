use secenly::application;
use secenly::exceptions::LicenseError;

fn main() -> Result<(), LicenseError> {
    println!("\n\n\n\n=====================================================================");
    println!("======================== WELCOME TO SECENLY! ========================");
    println!("=====================================================================\n");
    application::run()?;
    Ok(())
}