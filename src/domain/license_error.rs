use std::fmt;
use std::io;

use openssl::error::ErrorStack;
use rasn::ber::enc::Error as RasnError;

#[derive(Debug)]
pub enum LicenseError {
    Io(io::Error),
    OpenSsl(ErrorStack),
    DerEncoding(RasnError)
}

// Cubre los tipos de error io: archivos, escritura...
impl From<io::Error> for LicenseError {
    fn from(e: io::Error) -> Self {
        LicenseError::Io(e)
    }
}

// Cubre los tipos de error openssl: crypto, CMS...
impl From<ErrorStack> for LicenseError {
    fn from(e: ErrorStack) -> Self {
        LicenseError::OpenSsl(e)
    }
}

// Cubre los errores de ASN.1
impl From<RasnError> for LicenseError {
    fn from(err: RasnError) -> Self {
        LicenseError::DerEncoding(err)
    }
}

impl fmt::Display for LicenseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LicenseError::Io(e) =>
                write!(f, "File system error: {}", e),

            LicenseError::OpenSsl(e) =>
                write!(f, "Cryptographic error: {}", e),

            LicenseError::DerEncoding(e) =>
                write!(f, "DER encoding error: {}", e),
        }
    }
}

impl std::error::Error for LicenseError {}
