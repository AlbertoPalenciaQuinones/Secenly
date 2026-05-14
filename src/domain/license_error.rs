use std::fmt;
use std::io;

use chrono::ParseError;
use openssl::error::ErrorStack;
use rasn::ber::enc::Error as RasnError;

#[derive(Debug)]
pub enum LicenseError {
    // Error genérico del sistema de archivos (lectura/escritura básica sin contexto)
    Io(io::Error),
    // Error proveniente de Openssl (criptografía, certificados, CMS, etc)
    OpenSsl(ErrorStack),
    // Error en operaciones con la base de datos
    DatabaseError(rusqlite::Error),
    // Errores al codificar o decodificar en DER
    DerEncoding(RasnError),
    DerDecoding(String),
    // Error al parsear o validar fechas (entrada inválida o formato incorrecto)
    InvalidDate(String),
    // Indica que una operación esperaba elementos pero la lista está vacía
    EmptyList,
    // Error de lectura de archivo con contexto adicional (incluye ruta)
    IoWithContext {
        source: io::Error,
        path: String,
    },
    // Error al escribir en archivo con contexto adicional (incluye ruta)
    WriteWithContext {
        source: io::Error,
        path: String,
    },
    // Errores de certificados y clave (formato incorrecto, corrupto o no válido
    // para uso esperado
    InvalidCertificate {
        msg: String,
        source: Option<ErrorStack>,
    },
    InvalidPrivateKey {
        msg: String,
        source: Option<ErrorStack>,
    },
    // Error durante la generación de hashes (problemas criptográficos o de datos)
    HashError {
        msg: String,
        source: Option<ErrorStack>,
    },
    // Error relacionado con la obtención o generación del HWID
    HardwareError {
        msg: String,
    },
}

impl From<io::Error> for LicenseError {
    fn from(e: io::Error) -> Self {
        LicenseError::Io(e)
    }
}

impl From<ErrorStack> for LicenseError {
    fn from(e: ErrorStack) -> Self {
        LicenseError::OpenSsl(e)
    }
}

impl From<RasnError> for LicenseError {
    fn from(err: RasnError) -> Self {
        LicenseError::DerEncoding(err)
    }
}

impl From<rusqlite::Error> for LicenseError {
    fn from(err: rusqlite::Error) -> Self {
        LicenseError::DatabaseError(err)
    }
}

impl From<rasn::ber::de::Error> for LicenseError {
    fn from(err: rasn::ber::de::Error) -> Self {
        LicenseError::DerDecoding(err.to_string())
    }
}

impl From<ParseError> for LicenseError {
    fn from(err: ParseError) -> Self {
        LicenseError::InvalidDate(err.to_string())
    }
}

impl fmt::Display for LicenseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LicenseError::Io(e) =>
                write!(f, "File system error: {}", e),

            LicenseError::OpenSsl(e) =>
                write!(f, "Cryptographic error with openssl: {}", e),

            LicenseError::DerEncoding(e) =>
                write!(f, "DER encoding error: {}", e),

            LicenseError::DatabaseError(e) =>
                write!(f, "Database error: {}", e),

            LicenseError::DerDecoding(e) =>
                write!(f, "Decode error: {}", e),

            LicenseError::InvalidDate(e) =>
                write!(f, "Invalid date error: {}", e),

            LicenseError::EmptyList => 
                write!(f, "No licenses found"),
            
            LicenseError::IoWithContext { source, path } => {
                write!(f, "File error '{}': {}", path, source)}

            LicenseError::WriteWithContext { source, path } => {
                write!(f, "Write error at '{}': {}", path, source)}
                
            LicenseError::InvalidCertificate { msg, source: _ } =>
                write!(f, "Invalid certificate: {}", msg),

            LicenseError::InvalidPrivateKey { msg, source: _ } =>
                write!(f, "Invalid private key: {}", msg),

            LicenseError::HardwareError { msg } =>
                write!(f, "Hardware ID error: {}", msg),

            LicenseError::HashError { msg, source: _ } =>
                write!(f, "Hash generation error: {}", msg),

        }
    }
}

impl std::error::Error for LicenseError {}
