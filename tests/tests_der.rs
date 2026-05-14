#[cfg(test)]

use chrono::{Duration, Utc};
use std::path::PathBuf;

use secenly::builder::{builder::Builder, director::Director, license_builder::LicenseBuilder};
use secenly::domain::license_asn1::LicenseAsn1;
use secenly::services::{hardware_manager::HardwareManager, license_manager::LicenseManager, product_manager::ProductManager};

mod tests_it3 {
    use super::*;

    /* Test para verificar que la codificación DER de la licencia preserva los datos,
       al codificar con DER, los datos al decodificarlos deben ser iguales que antes,
       es decir, que los valores sean idénticos. Para comprobar la longitud antes de
       codificar con DER y después de decodificar, se utiliza un id de licencia generado
       anteriormente de 128 caracteres */
    #[test]
    fn der_roundtrip_preserves_data() {
        let license_manager = setup_license_manager();

        let mut license_builder = LicenseBuilder::default();
        Director::construct_license(&mut license_builder, 
            license_manager.get_license_id().to_string(), 
            Utc::now() + Duration::days(30), 
            60, 
            "roundtrip".to_string());

        let license = license_builder.build();

        let asn1 = LicenseAsn1::from(&license);
        let der = rasn::der::encode(&asn1).unwrap();
        let decoded: LicenseAsn1 = rasn::der::decode(&der).unwrap();

        assert_eq!(decoded.id.len(), 128);
        assert_eq!(decoded.heartbeat_interval, 60);
        assert_eq!(decoded.notes, "roundtrip");
    }

    /* Test para verificar que la codificación DER de la licencia es determinística,
       clonando una licencia y haciendo a esas 2 las codificación DER. Si coinciden
       en todos los bytes, entonces, es determinista */
    #[test]
    fn der_is_deterministic() {
        let license_manager = setup_license_manager();

        let mut license_builder = LicenseBuilder::default();
        Director::construct_license(&mut license_builder, 
            license_manager.get_license_id().to_string(), 
            Utc::now() + Duration::days(30), 
            60, 
            "roundtrip".to_string());

        let license1 = license_builder.build();

        let license2 = license1.clone();

        let der1 = rasn::der::encode(&LicenseAsn1::from(&license1)).unwrap();
        let der2 = rasn::der::encode(&LicenseAsn1::from(&license2)).unwrap();

        assert_eq!(der1, der2);
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