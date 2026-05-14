use crate::domain::license::License;

use rasn::{AsnType, Encode, Decode};
use rasn::types::{GeneralizedTime};

/**
 * Representa la estructura en formato ASN.1 que tendrán las licencias. Los
 * derive permiten que la estructura sea serializable.
 *
 * Este objeto puede ser modificado por cualquiera que utilice la herramienta, 
 * añadiendo o eliminando los atributos que quiera
 * 
 * Debe saber que puede añadir nuevos atributos a la licencia, supone que
 * también se haga en la biblioteca de Secenly, siempre y cuando se haya optado
 * utilizarla a la hora de validar licencias.
 * 
 * Es obligatorio seguir coherencia con la biblioteca a la hora de validar las
 * licencias y la herramienta, ya que ambos deben manejar los mismos atributos.
 */
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
