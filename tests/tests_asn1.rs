#[cfg(test)]

use chrono::{Duration, Utc};

use secenly::license_builder::LicenseBuilder;
use secenly::director::Director;
use secenly::builder::Builder;
use secenly::license_asn1::LicenseAsn1;

mod tests_it2 {
    use super::*;

    /* Test para verificar que el ID de la licencia es una cadena de bytes de 
       64 elementos en formato ASN.1 */
    #[test]
    fn asn1_id_is_octet_string_64_bytes() {
        let id = [9u8; 64]; 

        let mut license_builder = LicenseBuilder::default();
        Director::construct_license(&mut license_builder, id, Utc::now() + Duration::days(30), 10, "Tests license".to_string());
        let license = license_builder.build();

        let asn1 = LicenseAsn1::from(&license);

        assert_eq!(asn1.id.len(), 64);
    }
}