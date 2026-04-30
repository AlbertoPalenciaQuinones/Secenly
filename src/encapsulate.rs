use crate::{license::License, license_asn1::LicenseAsn1};

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use openssl::pkey::Private;
use openssl::cms::{CmsContentInfo, CMSOptions};
use openssl::x509::X509;
use openssl::pkey::PKey;
use openssl::stack::StackRef;

pub struct EncapsulateService;

// openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 -out key.pem  ->  generar clave privada RSA de 2048 bits
// openssl req -new -x509 -key key.pem -out cert.pem -days 365  ->  generar certificado autofirmado con la clave privada

impl EncapsulateService {

    pub fn encapsulate_license(license: License) {
        let license_asn1 = LicenseAsn1::from(&license);

        let der_bytes = rasn::der::encode(&license_asn1)
            .expect("Error serializando License DER");
        
        Self::write_license_der("licenses/license.der", &der_bytes)
            .expect("Error escribiendo el archivo de licencia");

        let signed_data = Self::signed_data(der_bytes);

        Self::write_license_der("licenses/signedData.der", &signed_data.unwrap())
            .expect("Error escribiendo el archivo de licencia");
    }

    fn write_license_der(path: &str, der_bytes: &[u8]) -> std::io::Result<()> {
        let path = Path::new(path);
        let mut file = File::create(path)?;
        file.write_all(der_bytes)?;
        Ok(())
    }

    fn signed_data(license_der_bytes: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let private_key_pem = fs::read("key.pem")
            .expect("No se pudo leer la clave privada");

        let certificate_pem = fs::read("cert.pem")
            .expect("No se pudo leer el certificado");

        let cert: X509 = X509::from_pem(&certificate_pem).expect("No se pudo crear el certificado");
        let key: PKey<Private> = PKey::private_key_from_pem(&private_key_pem).expect("No se pudo crear la clave privada");

        let cms = CmsContentInfo::sign(
            Option::Some(&cert),
            Some(&key),
            None::<&StackRef<X509>>,
            Some(license_der_bytes.as_ref()),
            CMSOptions::BINARY,
        )?;

        let der: Vec<u8> = cms.to_der()?;

        Ok(der)
    }
}

