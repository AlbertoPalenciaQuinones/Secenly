use std::fs::{self, File};
use std::io::{Write};
use std::path::PathBuf;

use crate::domain::{License, LicenseAsn1, LicenseError};

use openssl::cms::{CmsContentInfo, CMSOptions};
use openssl::pkey::{PKey, Private};
use openssl::{x509::X509};
use openssl::x509::store::X509StoreBuilder;

pub struct EncapsulateService;

// Encapsular la licencia en un ContentInfo codificándolo en DER.
impl EncapsulateService {
    // Desarrolla la generación de licencia en DER, siguiendo el flujo:
    // licencia (ASN.1) -> DER -> SignedData -> DER -> ContentInfo -> DER
    pub fn encapsulate_license(license: License, 
                               license_path: &PathBuf, 
                               cert_path: &PathBuf, 
                               key_path: &PathBuf
        ) -> Result<Vec<u8>, LicenseError> {
        // Obtener la licencia en ASN.1
        let license_asn1 = LicenseAsn1::from(&license);

        // Codificarla en formato DER
        let der_bytes = rasn::der::encode(&license_asn1)?;

        // Generar el SignedData de la licencia
        let signed_data = Self::signed_data(
            der_bytes, 
            cert_path, 
            key_path)?;

        // Escribir la licencia en der como un archivo
        Self::write_license_der(license_path, &signed_data)?;

        Ok(signed_data)
    }

    // Firma la licencia, generando el SignedData y la encapsula en un ContentInfo
    fn signed_data(license_der_bytes: Vec<u8>, 
                   cert_path: &PathBuf, 
                   key_path: &PathBuf
                   ) -> Result<Vec<u8>, LicenseError> {

        // Leer el certificado y la clave privada 
        let certificate_pem = fs::read(cert_path)
            .map_err(|e| LicenseError::IoWithContext {
                source: e,
                path: cert_path.display().to_string()
            })?;

        let private_key_pem = fs::read(key_path)
            .map_err(|e| LicenseError::IoWithContext {
                source: e,
                path: key_path.display().to_string(),
            })?;

        // Convierte el certificado y clave para que sean manejables
        let cert: X509 = X509::from_pem(&certificate_pem)
            .map_err(|e| LicenseError::InvalidCertificate {
                msg: "Invalid PEM certificate".into(),
                source: Some(e),
            })?;

        let key: PKey<Private> = PKey::private_key_from_pem(&private_key_pem)
            .map_err(|e| LicenseError::HashError {
                msg: "Invalid PEM key".into(),
                source: Some(e),
            })?;


        // Convertir la licencia al SignedData y a su vez, encapsular en ContentInfo
        let cms = CmsContentInfo::sign(
            Some(&cert),
            Some(&key),
            None,
            Some(&license_der_bytes),
            CMSOptions::BINARY,
        )?;

        // Codificarlo en DER para tener el archivo de licencia codificado y completo
        Ok(cms.to_der()?)
    }

    fn write_license_der(path: &PathBuf, der_bytes: &[u8]) -> Result<(), LicenseError> {
        // Crear el archivo de licencia
        let mut file = File::create(path)
            .map_err(|e| LicenseError::WriteWithContext {
                source: e,
                path: path.display().to_string(),
            })?;

        // Escribir en el todos los bytes de la licencia
        file.write_all(der_bytes) 
            .map_err(|e| LicenseError::WriteWithContext {
                source: e,
                path: path.display().to_string(),
            })?;

        Ok(())
    }

    pub fn decapsulate_license(der_bytes: &[u8]) -> Result<License, LicenseError> {
        // Parsear CMS
        let mut cms = CmsContentInfo::from_der(&der_bytes)
            .map_err(|e| LicenseError::DerDecoding(
                format!("Invalid CMS structure: {}", e)
            ))?;

        let store = X509StoreBuilder::new()?.build();
        let mut extracted = Vec::new();

        // Extraer el contenido interno
        cms.verify(
            None,
            Some(&store),
            None,
            Some(&mut extracted),
            CMSOptions::BINARY
                | CMSOptions::NO_SIGNER_CERT_VERIFY
                | CMSOptions::NO_ATTR_VERIFY,
        )?;

        // Decodificar ASN1 correcto
        let license_asn1: LicenseAsn1 = rasn::der::decode(&extracted)
            .map_err(|e| LicenseError::DerDecoding(
                format!("Invalid license ASN.1: {}", e)
            ))?;

        Ok(license_asn1.into())
    }
}