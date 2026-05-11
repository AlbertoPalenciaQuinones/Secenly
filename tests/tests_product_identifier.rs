#[cfg(test)]

use std::path::PathBuf;

use secenly::services::product_manager::ProductIdentifier;

mod tests_it4 {
    use super::*;

    /* Test para verificar que al generar un identificador de producto utilizando
       una semilla de ejemplo generada para los tests, se genere correctamente
       comprobando que no se genera vacío */ 
    #[test]
    fn test_product_identifier_generation() -> Result<(), Box<dyn std::error::Error>> {
        let path = PathBuf::from("example");
        let hwid = "TEST_HWID".to_string();

        let product = ProductIdentifier::initialize(&path, hwid)?;

        let product_id = product.get_product_id();

        assert!(!product_id.is_empty());

        Ok(())
    }

    /* Test para verificar que la generación del identificador de producto sea
       determinista. Es decir, al generar 2 identificadores de producto con la
       misma semilla y identificador de hardware, que ambos sean iguales */
    #[test]
    fn test_product_identifier_deterministic() -> Result<(), Box<dyn std::error::Error>> {
        let path = PathBuf::from("example");
        let hwid = "TEST_HWID".to_string();

        let p1 = ProductIdentifier::initialize(&path, hwid.clone())?;
        let p2 = ProductIdentifier::initialize(&path, hwid)?;

        assert_eq!(p1.get_product_id(), p2.get_product_id());

        Ok(())
    }

}