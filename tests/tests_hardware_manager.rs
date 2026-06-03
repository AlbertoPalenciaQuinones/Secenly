#[cfg(test)]

use secenly::services::hardware_manager::HardwareManager;

mod tests_it4 {
    use super::*;

    /* Test para verificar que la generación del identificador de hardware
       se realiza correctamente. Se comprueba que la creación del
       HardwareManager no produce errores y que el identificador obtenido
       no está vacío. */
    #[test]
    fn test_hardware_identifier_generation() {
        let hardware_manager = HardwareManager::new();

        assert!(hardware_manager.is_ok());

        let hw = hardware_manager.unwrap();

        assert!(!hw.get_hardware_id().is_empty());
    }

    /* Test para verificar que el identificador de hardware generado está
       correctamente formateado, sin contener espacios en blanco ni saltos
       de línea innecesarios. */
    #[test]
    fn test_hwid_trimmed() {
        let manager = HardwareManager::new().unwrap();

        let hwid = manager.get_hardware_id();

        assert_eq!(hwid, hwid.trim());
    }
}