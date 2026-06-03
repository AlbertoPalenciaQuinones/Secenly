#[cfg(test)]

use chrono::{Duration, Utc};

use secenly::builder::license_builder::LicenseBuilder;
use secenly::builder::director::Director;
use secenly::builder::builder::Builder;

mod tests_it2 {
    use super::*;

    /* Test para verificar que las fechas de la licencia son correctas.
       Se verifica que la fecha de expiración sea mayor a la fecha de creación. */
    #[test]
    fn builds_valid_license() {
        let expiration = Utc::now() + Duration::days(30);

        let mut license_builder = LicenseBuilder::default();
        Director::construct_license(&mut license_builder, String::from("Test ID"), expiration, 10, "Tests license".to_string());
        let license = license_builder.build();

        assert!(license.expiration_date > license.creation_date);
    }

    /* Test para verificar que las licencias se crean con fechas correctas,
       verificando que la fecha obtenida antes de crear la licencia es menor
       o igual a la fecha de creación de la licencia, y que la fecha obtenida 
       después de crear la licencia es mayor o igual a la fecha de creación de la licencia. */
    #[test]
    fn creation_date_is_nowish() {
        let before = Utc::now();

        let mut license_builder = LicenseBuilder::default();
        Director::construct_license(&mut license_builder, 
            String::from("TEST ID"), 
            before + Duration::days(30), 
            10, 
            "Tests license".to_string());
        
        let license = license_builder.build();

        let after = Utc::now();

        // Se la añade un margen de error de 1 segundo para evitar fallos por pequeñas diferencias
        assert!(license.creation_date >= before - Duration::seconds(1));
        assert!(license.creation_date <= after + Duration::seconds(1));
    }

    /* Test para verificar que el sistema es capaz de manejar correctamente
       notas de gran tamaño en una licencia. Se genera una cadena extensa y
       se asigna como campo de notas, comprobando posteriormente que el valor
       se mantiene íntegro tras la construcción de la licencia. */
    #[test]
    fn handles_large_notes() {
        let large_notes = "A".repeat(1000);

        let mut builder = LicenseBuilder::default();

        Director::construct_license(
            &mut builder,
            "test_id".to_string(),
            Utc::now() + Duration::days(30),
            10,
            large_notes.clone(),
        );

        let license = builder.build();

        assert_eq!(license.notes, large_notes);
    }

    /* Test para verificar que el patrón de diseño Builder utilizado en la
       construcción de licencias produce objetos válidos. Se crea una licencia
       mediante el Director y el LicenseBuilder, comprobando que al menos uno
       de sus campos principales (el identificador) contiene un valor válido. */
    #[test]
    fn builder_produces_valid_license() {
        let mut builder = LicenseBuilder::default();

        Director::construct_license(
            &mut builder,
            "test_id".to_string(),
            Utc::now() + Duration::days(30),
            30,
            "Notes".to_string(),
        );

        let license = builder.build();

        assert!(!license.id.is_empty());
    }
}