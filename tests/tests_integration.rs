#[cfg(test)]

use std::path::PathBuf;

use secenly::application::obtain_licenses;
use secenly::builder::director::Director;
use secenly::builder::builder::Builder;
use secenly::builder::license_builder::LicenseBuilder;
use secenly::services::hardware_manager::HardwareManager;
use secenly::services::license_manager::LicenseManager;
use secenly::services::product_manager::ProductManager;
use secenly::services::encapsulate::EncapsulateService;

use chrono::Utc;
use rusqlite::{Connection};

mod tests_it6 {
    use super::*;

    /* Test para verificar que el flujo de creación de licencias funciona
       correctamente. Para ello, se recrea el flujo de crear licencias exactamente
       igual que en la funcionalidad de la clase principal. */
    #[test]
    fn test_license_creation() {
        let license_path = PathBuf::from("tests/resources/license_integration.der");
        let seed_path = PathBuf::from("tests/resources/seed.dat");
        let cert_path = PathBuf::from("tests/resources/cert.pem");
        let key_path = PathBuf::from("tests/resources/key.pem");

        let hardware_manager = HardwareManager::new().expect("ERROR");
        let product_manager = ProductManager::new(
            &seed_path, hardware_manager.get_hardware_id()).expect("ERROR");
        let license_manager = LicenseManager::new(
            &product_manager.get_product_id()).expect("ERROR");

        let mut license_builder = LicenseBuilder::default();

        // Construcción de la licencia
        Director::construct_license(&mut license_builder, 
            license_manager.get_license_id().to_string(), 
            Utc::now(),
            60, 
            "TEST_LICENSE".to_string()
        );

        let license = license_builder.build();

        let der_bytes = EncapsulateService::encapsulate_license(
            license, 
            &license_path, 
            &cert_path, 
            &key_path
        ).expect("ERROR");
    }

    /* Test para verificar que el flujo de lectura y decodificación de licencias 
       funciona correctamente. Para ello, se obtienen las licencias de la bbdd 
       y se decodifican. */
    #[test]
    fn test_read_and_decode_flow() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE licenses (
                id INTEGER PRIMARY KEY, 
                hwid TEXT, 
                seed BLOB, 
                license_der BLOB
            )",
            [],
        ).unwrap();

        let licenses = obtain_licenses(&conn, None).unwrap();

        for (_, der) in licenses {
            let license = EncapsulateService::decapsulate_license(&der).unwrap();

            assert!(!license.id.is_empty());
        }
    }
}