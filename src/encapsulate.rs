use crate::{license::License, license_asn1::LicenseAsn1};

use std::fs::File;
use std::io::Write;
use std::path::Path;

use cryptographic_message_syntax::SignedDataBuilder;

pub struct EncapsulateService;

impl EncapsulateService {

    pub fn encapsulate_license(license: License) {
        let license_asn1 = LicenseAsn1::from(&license);

        let der_bytes = rasn::der::encode(&license_asn1)
            .expect("Error serializando License DER");

        println!("{:?}", license);
        println!("DER bytes: {:02x?}", der_bytes);
        
        Self::write_license_der("licenses/license.der", &der_bytes)
            .expect("Error escribiendo el archivo de licencia");

        let signed_data = Self::signed_data(der_bytes);
    }

    fn write_license_der(path: &str, der_bytes: &[u8]) -> std::io::Result<()> {
        let path = Path::new(path);
        let mut file = File::create(path)?;
        file.write_all(der_bytes)?;
        Ok(())
    }

    fn signed_data(license_der_bytes: Vec<u8>) {
        let cms_der = SignedDataBuilder::default()
                        .content_inline(license_der_bytes)
                        .build_der()
                        .expect("Error construyendo CMS SignedData");
        println!("CMS SignedData DER: {:02x?}", cms_der);
    }
}

