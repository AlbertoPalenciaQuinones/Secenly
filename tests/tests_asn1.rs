#[cfg(test)]

use chrono::{Duration, Utc};

use secenly::license_asn1::LicenseAsn1;
use secenly::license_manager::obtain_hwid;
use secenly::license_id_generator::LicenseIdGenerator;
use secenly::license_builder::LicenseBuilder;
use secenly::director::Director;
use secenly::builder::Builder;

mod tests_it3 {
    use super::*;

    /* Test para verificar que el ID de la licencia es una cadena de 
       de 128 caracteres en formato ASN.1. Este test se ha modificado
       en cuanto a la iteración 2 porque el formato de ASN.1 ha cambiado.
       Se ha generado una seed aleatoria, se ha guardado en una variable
       y con ella se comprueba que genera esos 128 caracteres de id  */
    #[test]
    fn asn1_id_is_string_128_chars() {
        let seed_bytes = [48, 207, 181, 12, 88, 141, 232, 99, 127, 203, 249, 15, 43, 203, 70, 151, 146, 122, 229, 4, 160, 188, 247, 88, 73, 223, 101, 106, 248, 129, 136, 200, 217, 31, 244, 232, 76, 182, 173, 107, 104, 129, 133, 61, 199, 4, 35, 246, 189, 82, 27, 84, 235, 121, 112, 72, 218, 101, 211, 56, 158, 253, 60, 255, 37, 54, 23, 53, 10, 242, 126, 233, 151, 201, 128, 231, 230, 16, 243, 46, 39, 75, 94, 161, 110, 133, 156, 87, 45, 104, 44, 221, 162, 199, 41, 84, 194, 70, 253, 133, 253, 66, 173, 122, 220, 118, 249, 85, 100, 179, 126, 253, 245, 0, 162, 195, 23, 255, 201, 8, 218, 210, 39, 221, 2, 213, 93, 50, 153, 80, 206, 237, 106, 19, 238, 185, 4, 13, 124, 50, 115, 113, 223, 247, 236, 118, 6, 1, 222, 138, 73, 163, 217, 21, 176, 217, 106, 80, 94, 182, 226, 118, 183, 110, 52, 146, 14, 226, 165, 139, 64, 19, 17, 6, 14, 194, 161, 237, 148, 58, 27, 144, 192, 80, 36, 102, 232, 1, 96, 21, 188, 132, 1, 26, 129, 62, 89, 129, 41, 180, 54, 48, 251, 89, 47, 18, 73, 196, 154, 190, 190, 45, 201, 118, 107, 120, 20, 43, 34, 43, 194, 251, 49, 99, 206, 123, 217, 106, 187, 170, 161, 10, 28, 90, 167, 113, 68, 235, 61, 137, 20, 55, 117, 123, 43, 54, 77, 216, 224, 238, 98, 107, 110, 21, 219, 216];

        let mut product_id = obtain_hwid();

        for byte in seed_bytes.iter() {
            product_id += &format!("{}", *byte as u32);
        }
        let license_id = LicenseIdGenerator::generate(product_id.as_str());

        let license = LicenseAsn1 {
            id: license_id,
            creation_date: "2024-01-01T00:00:00Z".parse().unwrap(),
            expiration_date: "2024-12-31T23:59:59Z".parse().unwrap(),
            last_use_date: "2024-06-01T12:00:00Z".parse().unwrap(),
            heartbeat_interval: 60,
            notes: String::from("Test license"),
        };

        assert_eq!(license.id.len(), 128);
    }

    /* Test para verificar que al generar una licencia y pasarla
       a formato ASN.1, los campos no están vacíos */
    #[test]
    fn asn1_license_fields_not_empty() {
        let mut license_builder = LicenseBuilder::default();
        Director::construct_license(&mut license_builder, String::from("f9b119a6d580c6040c1b7d8635b2b2fcb221c1f07d8977c8122e7e4b3527f0724cdb7c86049b2a84684bc401228558ad1e8aec4c8e7887e701425c31ab4d8077"), Utc::now() + Duration::days(30), 60, "Test license".to_string());
        let license = license_builder.build();

        let asn1 = LicenseAsn1::from(&license);

        // ID
        assert_eq!(!asn1.id.is_empty(), true);
       
       // Creation date
        assert_eq!(asn1.creation_date.to_string().is_empty(), false);

        // Expiration date
        assert_eq!(asn1.expiration_date.to_string().is_empty(), false);

        // Last use date
        assert_eq!(asn1.last_use_date.to_string().is_empty(), false);

        // Heartbeat interval
        assert_eq!(asn1.heartbeat_interval > 0, true);

        // Notes
        assert_eq!(!asn1.notes.is_empty(), true);
    }

    #[test]
    fn license_raw_to_asn1_correct_mapping() {
        let id = String::from("f9b119a6d580c6040c1b7d8635b2b2fcb221c1f07d8977c8122e7e4b3527f0724cdb7c86049b2a84684bc401228558ad1e8aec4c8e7887e701425c31ab4d8077");

        let mut license_builder = LicenseBuilder::default();
        Director::construct_license(&mut license_builder, id, Utc::now() + Duration::days(30), 60, "roundtrip".to_string());
        let license = license_builder.build();
        let asn1 = LicenseAsn1::from(&license);

        // ID
        assert_eq!(asn1.id, license.id);
        
        // Creation date
        assert_eq!(asn1.creation_date, license.creation_date);

        // Expiration date
        assert_eq!(asn1.expiration_date, license.expiration_date);

        // Last use date
        assert_eq!(asn1.last_use_date, license.last_use_date);

        // Heartbeat interval
        assert_eq!(asn1.heartbeat_interval, license.heartbeat_interval);

        // Notes
        assert_eq!(asn1.notes, license.notes);
    }
}