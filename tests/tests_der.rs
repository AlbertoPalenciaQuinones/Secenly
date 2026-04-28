#[cfg(test)]

use chrono::{Duration, Utc};

use secenly::license_builder::LicenseBuilder;
use secenly::director::Director;
use secenly::builder::Builder;
use secenly::license_asn1::LicenseAsn1;

mod tests_it2 {
    use super::*;

    /* Test para verificar que la codificación DER de la licencia preserva los datos,
       al codificar con DER, los datos al decodificarlos deben ser iguales que antes,
       es decir, que los valores sean idénticos */
    #[test]
    fn der_roundtrip_preserves_data() {
        let mut license_builder = LicenseBuilder::default();
        Director::construct_license(&mut license_builder, [7u8; 64], Utc::now() + Duration::days(30), 60, "roundtrip".to_string());
        let license = license_builder.build();

        let asn1 = LicenseAsn1::from(&license);
        let der = rasn::der::encode(&asn1).unwrap();
        let decoded: LicenseAsn1 = rasn::der::decode(&der).unwrap();

        assert_eq!(decoded.id.len(), 64);
        assert_eq!(decoded.heartbeat_interval, 60);
        assert_eq!(decoded.notes, "roundtrip");
    }

    /* Test para verificar que la codificación DER de la licencia es determinística,
       clonando una licencia y haciendo a esas 2 las codificación DER. Si coinciden
       en todos los bytes, entonces, es determinista */
    #[test]
    fn der_is_deterministic() {
        let id = [5u8; 64];

        let mut license_builder = LicenseBuilder::default();
        Director::construct_license(&mut license_builder, id, Utc::now() + Duration::days(30), 60, "roundtrip".to_string());
        let license1 = license_builder.build();

        let license2 = license1.clone();

        let der1 = rasn::der::encode(&LicenseAsn1::from(&license1)).unwrap();
        let der2 = rasn::der::encode(&LicenseAsn1::from(&license2)).unwrap();

        assert_eq!(der1, der2);
    }
}