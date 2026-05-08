#[cfg(test)]

use chrono::{Duration, Utc};

use secenly::license_builder::LicenseBuilder;
use secenly::director::Director;
use secenly::builder::Builder;

mod tests_it2 {
    use super::*;

    /* Test para verificar que las fechas de la licencia son correctas.
       Se verifica que la fecha de expiración sea mayor a la fecha de creación */
    #[test]
    fn builds_valid_license() {
        let expiration = Utc::now() + Duration::days(30);

        let mut license_builder = LicenseBuilder::default();
        Director::construct_license(&mut license_builder, [0u8; 64], expiration, 10, "Tests license".to_string());
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
        Director::construct_license(&mut license_builder, [0u8; 64], before + Duration::days(30), 10, "Tests license".to_string());
        let license = license_builder.build();

        let after = Utc::now();

        assert!(license.creation_date >= before);
        assert!(license.creation_date <= after);
    }
}