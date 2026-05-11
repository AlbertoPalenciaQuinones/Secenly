use crate::domain::license::License;

use rasn::{AsnType, Encode, Decode};
use rasn::types::{GeneralizedTime};

// Representa la estructura en formato ASN.1 que tendrán las licencias. Los
// derive permiten que la estructura sea serializable.
#[derive(AsnType, Encode, Decode, Debug)]
pub struct LicenseAsn1 {
    // El identificador es un OCTET STRING
    pub id: String,                      
    pub creation_date: GeneralizedTime, 
    pub expiration_date: GeneralizedTime,
    pub heartbeat_interval: i32,
    pub notes: String,
}

impl From<&License> for LicenseAsn1 {
    fn from(license: &License) -> Self {
        Self {
            id: license.id.clone(),
            creation_date: GeneralizedTime::from(license.creation_date),
            expiration_date: GeneralizedTime::from(license.expiration_date),
            heartbeat_interval: license.heartbeat_interval,
            notes: license.notes.clone(),
        }
    }
}
