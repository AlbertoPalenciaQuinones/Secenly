#[cfg(test)]

use secenly::domain::license_error::LicenseError;


/* Conjunto de pruebas unitarias destinadas a verificar el correcto funcionamiento
   del sistema de gestión de errores del proyecto. Estos tests comprueban tanto la
   creación directa de los distintos tipos de errores como su generación a partir
   de conversiones automáticas.

   El objetivo de estas pruebas es garantizar que cada tipo de error  se produce en 
   las condiciones adecuadas*/
mod tests_it3 {
    use super::*;

    #[test]
    fn test_empty_list_error() {
        let result = Err::<(), LicenseError>(LicenseError::EmptyList);

        assert!(matches!(result, Err(LicenseError::EmptyList)));
    }
    
    #[test]
    fn test_invalid_date_from_parse() {
        let date_result = "invalid-date".parse::<chrono::NaiveDate>();

        let error: LicenseError = date_result.unwrap_err().into();

        assert!(matches!(error, LicenseError::InvalidDate(_)));
    }
    
    #[test]
    fn test_database_error() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();

        let result = conn.execute("INVALID SQL", []);

        let error: LicenseError = result.unwrap_err().into();

        assert!(matches!(error, LicenseError::DatabaseError(_)));
    }

    #[test]
    fn test_io_error() {
        let result = std::fs::read("file_that_does_not_exist.txt");

        let error: LicenseError = result.unwrap_err().into();

        assert!(matches!(error, LicenseError::Io(_)));
    }
    
    #[test]
    fn test_io_with_context() {
        let error = LicenseError::IoWithContext {
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
            path: "some/path".to_string(),
        };

        assert!(matches!(error, LicenseError::IoWithContext { .. }));
    }
    
    #[test]
    fn test_write_with_context() {
        let error = LicenseError::WriteWithContext {
            source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "no permission"),
            path: "some/path".to_string(),
        };

        assert!(matches!(error, LicenseError::WriteWithContext { .. }));
    }
    
    #[test]
    fn test_invalid_certificate_error() {
        let error = LicenseError::InvalidCertificate {
            msg: "Invalid cert".into(),
            source: None,
        };

        assert!(matches!(error, LicenseError::InvalidCertificate { .. }));
    }
    
    #[test]
    fn test_invalid_private_key_error() {
        let error = LicenseError::InvalidPrivateKey {
            msg: "Invalid key".into(),
            source: None,
        };

        assert!(matches!(error, LicenseError::InvalidPrivateKey { .. }));
    }
    
    #[test]
    fn test_hash_error() {
        let error = LicenseError::HashError {
            msg: "Hash failed".into(),
            source: None,
        };

        assert!(matches!(error, LicenseError::HashError { .. }));
    }

    #[test]
    fn test_hardware_error() {
        let error = LicenseError::HardwareError {
            msg: "Unsupported OS".into(),
        };

        assert!(matches!(error, LicenseError::HardwareError { .. }));
    }
    
    #[test]
    fn test_der_decoding_error() {
        let error = LicenseError::DerDecoding("decode failed".into());

        assert!(matches!(error, LicenseError::DerDecoding(_)));
    }
}
