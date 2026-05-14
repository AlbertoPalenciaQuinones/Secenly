#[cfg(test)]

use chrono::{Duration, Utc};
use std::path::PathBuf;

use secenly::domain::license_asn1::LicenseAsn1;
use secenly::services::{hardware_manager::HardwareManager, license_manager::LicenseManager, product_manager::ProductManager};
use secenly::builder::{license_builder::LicenseBuilder};
use secenly::builder::director::Director;
use secenly::builder::builder::Builder;

mod tests_it3 {
    use super::*;

    /* Test para verificar que el ID de la licencia es una cadena de 
       de 128 caracteres en formato ASN.1. Este test se ha modificado
       en cuanto a la iteración 2 porque el formato de ASN.1 ha cambiado.
       Se ha generado una seed aleatoria, se ha guardado en una variable
       y con ella se comprueba que genera esos 128 caracteres de id  */
    #[test]
    fn asn1_id_is_string_128_chars() {
        let license_manager = setup_license_manager();

        let license = LicenseAsn1 {
            id: license_manager.get_license_id().to_string(),
            creation_date: "2024-01-01T00:00:00Z".parse().unwrap(),
            expiration_date: "2024-12-31T23:59:59Z".parse().unwrap(),
            heartbeat_interval: 60,
            notes: String::from("Test license"),
        };

        assert_eq!(license.id.len(), 128);
    }

    
    /* Test para verificar que al generar una licencia y transformarla a formato
       ASN.1, todos sus campos contienen valores válidos y no están vacíos. */
    #[test]
    fn asn1_license_fields_not_empty() {
        let license_manager = setup_license_manager();

        let mut license_builder = LicenseBuilder::default();
        Director::construct_license(&mut license_builder, 
            license_manager.get_license_id().to_string(), 
            Utc::now() + Duration::days(30), 
            60, 
            "Test license".to_string());

        let license = license_builder.build();

        let asn1 = LicenseAsn1::from(&license);

        // ID
        assert_eq!(!asn1.id.is_empty(), true);
       
       // Creation date
        assert_eq!(asn1.creation_date.to_string().is_empty(), false);

        // Expiration date
        assert_eq!(asn1.expiration_date.to_string().is_empty(), false);

        // Heartbeat interval
        assert_eq!(asn1.heartbeat_interval > 0, true);

        // Notes
        assert_eq!(!asn1.notes.is_empty(), true);
    }

    
    /* Test para verificar que la conversión de una licencia desde su forma original
       a su representación en ASN.1 se realiza de manera correcta, manteniendo todos 
       los valores de los campos. */
    #[test]
    fn license_raw_to_asn1_correct_mapping() {
        let license_manager = setup_license_manager();

        let mut license_builder = LicenseBuilder::default();
        Director::construct_license(&mut license_builder, 
            license_manager.get_license_id().to_string(), 
            Utc::now() + Duration::days(30), 
            60, 
            "roundtrip".to_string());

        let license = license_builder.build();
        let asn1 = LicenseAsn1::from(&license);

        // ID
        assert_eq!(asn1.id, license.id);
        
        // Creation date
        assert_eq!(asn1.creation_date, license.creation_date);

        // Expiration date
        assert_eq!(asn1.expiration_date, license.expiration_date);

        // Heartbeat interval
        assert_eq!(asn1.heartbeat_interval, license.heartbeat_interval);

        // Notes
        assert_eq!(asn1.notes, license.notes);
    }
}

// Función para establecer el objeto license_manager y evitar repeticiones
fn setup_license_manager() -> LicenseManager {
    let seed_path: PathBuf = "example/seed.dat".into();

    let hardware_manager = HardwareManager::new().expect("ERROR");

    let product_manager = ProductManager::new(&seed_path, 
        hardware_manager.get_hardware_id())
        .expect("ERROR");

    let license_manager = LicenseManager::new(product_manager.get_product_id())
        .expect("ERROR");

    license_manager
}