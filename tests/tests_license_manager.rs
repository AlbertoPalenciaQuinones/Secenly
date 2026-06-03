#[cfg(test)]

use secenly::services::{hardware_manager::HardwareManager, product_manager::ProductManager};
use secenly::services::license_manager::LicenseManager;

use std::path::PathBuf;

mod tests_it4 {
    use super::*;

    /* Test para verificar que al generar un identificador de licencia aleatorio
       utilizando un identificador de producto aleatorio, se genere correctamente
       comprobando que no se genera vacío */ 
    #[test]
    fn test_license_identifier() {
        let license_manager = setup_license_manager();

        let id = license_manager.get_license_id();

        assert!(!id.is_empty());

    }

    /* Test para verificar que el identificador de licencia generado se encuentra
       correctamente representado en formato hexadecimal. Se comprueba que todos
       los caracteres que componen el identificador pertenecen al conjunto de
       dígitos hexadecimales válidos. */
    #[test]
    fn test_license_identifier_is_hex() {
        let license_manager = setup_license_manager();

        let id = license_manager.get_license_id();

        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));

    }
}

// Función para establecer el objeto license_manager y evitar repeticiones
fn setup_license_manager() -> LicenseManager {
    let seed_path: PathBuf = "tests/resources/seed.dat".into();

    let hardware_manager = HardwareManager::new().expect("ERROR");

    let product_manager = ProductManager::new(&seed_path, 
        hardware_manager.get_hardware_id())
        .expect("ERROR");

    let license_manager = LicenseManager::new(product_manager.get_product_id())
        .expect("ERROR");

    license_manager
}