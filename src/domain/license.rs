use crate::domain::license_asn1::LicenseAsn1;

use chrono::{DateTime, Utc};

// Permite imprimir licencias en logs y depurar tests. Se utiliza la clonación
// en algunos tests.
#[derive(Debug, Clone)]

// Representa una licencia de software completamente construida.
pub struct License {    
    pub id: String,
    pub creation_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub heartbeat_interval: i32,
    pub notes: String,
}

/**
 * El objeto de la licencia.
 *
 * Este objeto puede ser modificado por cualquiera que utilice la herramienta, 
 * añadiendo o eliminando los atributos que quiera,
 * 
 * Debe saber que puede añadir nuevos atributos a la licencia, supone que
 * también se haga en la biblioteca Secenly, siempre y cuando se haya optado
 * utilizarla a la hora de validar licencias,
 * 
 * Es obligatorio seguir coherencia con la herramienta de generación de 
 * licencias y la biblioteca, ya que ambos deben manejar los mismos atributos.
 * 
 * En cuanto a la generación de licencia, se utiliza el patrón builder para
 * facilitar a quien quiera modificar los atributos realizar dicho cambio. Debe
 * saber que si modifica la estructura de la licencia, tendrá que modificar las
 * clases que corresponden al patrón: director, builder y license_builder.
 */
impl License {
    pub fn new(
        id: String,
        creation_date: DateTime<Utc>,
        expiration_date: DateTime<Utc>,
        heartbeat_interval: i32,
        notes: String, 
    ) -> License {
        License {
            id,
            creation_date,
            expiration_date,
            heartbeat_interval: heartbeat_interval,
            notes,
        }
    }

    /*pub fn set_id(&mut self, id: String) {
        self.id = id;
    }


    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn set_creation_date(&mut self, creation_date: DateTime<Utc>) {
        self.creation_date = creation_date;
    }

    pub fn get_creation_date(&self) -> &DateTime<Utc> {
        &self.creation_date
    }

    pub fn set_expiration_date(&mut self, expiration_date: DateTime<Utc>) {
        self.expiration_date = expiration_date;
    }

    pub fn get_expiration_date(&self) -> &DateTime<Utc> {
        &self.expiration_date
    }

    pub fn set_last_use_date(&mut self, last_use_date: DateTime<Utc>) {
        self.last_use_date = last_use_date;
    }

    pub fn get_last_use_date(&self) -> &DateTime<Utc> {
        &self.last_use_date
    }

    pub fn set_heartbeat_interval(&mut self, heartbeat_interval: i32) {
        self.heartbeat_interval = heartbeat_interval;
    }

    pub fn get_heartbeat_interval(&self) -> &i32 {
        &self.heartbeat_interval
    }

    pub fn set_notes(&mut self, notes: String) {
        self.notes = notes;
    }

    pub fn get_notes(&self) -> &String {
        &self.notes
    }*/
}

impl From<LicenseAsn1> for License {
    fn from(asn1: LicenseAsn1) -> Self {
        License {
            id: asn1.id.to_string(),
            creation_date: asn1.creation_date.into(),
            expiration_date: asn1.expiration_date.into(),
            heartbeat_interval: asn1.heartbeat_interval,
            notes: asn1.notes.to_string(),
        }
    }
}

