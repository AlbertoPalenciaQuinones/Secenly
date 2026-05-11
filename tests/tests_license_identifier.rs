#[cfg(test)]

use secenly::services::license_manager::LicenseIdentifier;

mod tests_it4 {
    use super::*;

    /* Test para verificar que al generar un identificador de licencia aleatorio
       utilizando un identificador de producto aleatorio, se genere correctamente
       comprobando que no se genera vacío */ 
    #[test]
    fn test_license_identifier() -> Result<(), Box<dyn std::error::Error>> {
        let product_id = "TEST_PRODUCT_ID".to_string();

        let license = LicenseIdentifier::initialize(&product_id)?; 

        let id = license.get_license_id();

        assert!(!id.is_empty());

        Ok(())
    }

    /* Test para verificar que el identificador de licencia se haya generado
       correctamente en hexadecimal */
    #[test]
    fn test_license_identifier_is_hex() -> Result<(), Box<dyn std::error::Error>> {
        let product_id = "TEST_PRODUCT_ID".to_string();

        let license = LicenseIdentifier::initialize(&product_id)?;

        let id = license.get_license_id();

        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));

        Ok(())
    }
}