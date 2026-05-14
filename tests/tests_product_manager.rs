#[cfg(test)]

use std::path::PathBuf;

use secenly::services::product_manager::ProductManager;

mod tests_it4 {
    use super::*;

    /* Test para verificar que al generar un identificador de producto utilizando
       una semilla de ejemplo generada para los tests, se genere correctamente
       comprobando que no se genera vacío */ 
    #[test]
    fn test_product_identifier_generation() {
        let path = PathBuf::from("example/seed.dat");
        let hwid = "TEST_HWID".to_string();

        let product_manager = ProductManager::new(&path, hwid).expect("ERROR");

        assert!(!product_manager.get_product_id().is_empty());
    }

    /* Test para verificar que la generación del identificador de producto sea
       determinista. Es decir, al generar 2 identificadores de producto con la
       misma semilla y identificador de hardware, que ambos sean iguales */
    #[test]
    fn test_product_identifier_deterministic() {
        let path = PathBuf::from("example/seed.dat");
        let hwid = "TEST_HWID".to_string();

        let p1 = ProductManager::new(&path, hwid.clone()).expect("ERROR");
        let p2 = ProductManager::new(&path, hwid).expect("ERROR");

        assert_eq!(p1.get_product_id(), p2.get_product_id());
    }

}