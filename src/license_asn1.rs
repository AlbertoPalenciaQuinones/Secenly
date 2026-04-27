use rasn::{AsnType, Encode, Decode};
use rasn::types::GeneralizedTime;
use crate::license::License;

#[derive(AsnType, Encode, Decode, Debug)]
pub struct LicenseAsn1 {
    pub id: Vec<u8>,                     // OCTET STRING
    pub creation_date: GeneralizedTime,  // GeneralizedTime
    pub expiration_date: GeneralizedTime,
    pub last_use_date: GeneralizedTime,
    pub heartbeat_interval: i32,
    pub notes: String,
}


impl From<&License> for LicenseAsn1 {
    fn from(license: &License) -> Self {
        Self {
            id: license.id.to_vec(),
            creation_date: GeneralizedTime::from(license.creation_date),
            expiration_date: GeneralizedTime::from(license.expiration_date),
            last_use_date: GeneralizedTime::from(license.last_use_date),
            heartbeat_interval: license.heartbeat_interval,
            notes: license.notes.clone(),
        }
    }
}
