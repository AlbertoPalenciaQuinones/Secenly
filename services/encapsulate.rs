use std::fs::{self, File};
use std::io::{Write};
use std::path::PathBuf;

use crate::domain::{License, LicenseAsn1, LicenseError};

use openssl::cms::{CmsContentInfo, CMSOptions};
use openssl::pkey::{PKey, Private};
use openssl::{x509::X509};

pub struct EncapsulateService;

// Encapsular la licencia en un ContentInfo codificándolo en DER.
impl EncapsulateService {
    // Desarrolla la generación de licencia en DER, siguiendo el flujo:
    // licencia (ASN.1) -> DER -> SignedData -> DER -> ContentInfo -> DER
    pub fn encapsulate_license(license: License, 
                               license_path: &PathBuf, 
                               cert_path: &PathBuf, 
                               key_path: &PathBuf
        ) -> Result<(), LicenseError> {
        // Obtener la licencia en ASN.1
        let license_asn1 = LicenseAsn1::from(&license);

        // Codificarla en formato DER
        let der_bytes = rasn::der::encode(&license_asn1)?;

        // Generar el SignedData de la licencia
        let signed_data = Self::signed_data(
            der_bytes, 
            cert_path, 
            key_path)?;

        Self::write_license_der(license_path, &signed_data)?;

        Ok(())
    }

    fn signed_data(license_der_bytes: Vec<u8>, 
                   cert_path: &PathBuf, 
                   key_path: &PathBuf
                   ) -> Result<Vec<u8>, LicenseError> {
        // Leer el certificado y la clave privada del archivo
        // LANZAR ERROR SI NO SE ENCUENTRA EL ARCHIVO
        let certificate_pem = fs::read(cert_path)?;
        let private_key_pem = fs::read(key_path)?;

        // Se pasan a un formato manejable 
        let cert: X509 = X509::from_pem(&certificate_pem)?;
        let key: PKey<Private> = PKey::private_key_from_pem(&private_key_pem)?;

        // Convertir la licencia al SignedData y a su vez, encapsular en ContentInfo
        let cms = CmsContentInfo::sign(
            Some(&cert),
            Some(&key),
            None,
            Some(&license_der_bytes),
            CMSOptions::BINARY,
        )?;

        // Codificarlo a der para tener el archivo de licencia codificado y completo
        Ok(cms.to_der()?)
    }

    fn write_license_der(path: &PathBuf, 
                         der_bytes: &[u8]
                         ) -> Result<(), LicenseError> {
        let mut file = File::create(path)?;
        file.write_all(der_bytes)?;
        Ok(())
    }
}

