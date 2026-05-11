use chrono::{DateTime, Timelike, Utc};

use crate::builder::builder::Builder;
pub struct Director;

// Centraliza el flujo del patrón builder para la generación de licencias
impl Director {
    
    pub fn construct_license(
        // Se modifica el estado interno del builder (mut = lectura + escritura)
        builder: &mut impl Builder, 
        id: String,
        expiration_date: DateTime<Utc>,
        heartbeat_interval: i32,
        notes: String,
        ) {

        builder.set_id(id);
        // La fecha de creación no la decide el cliente y se crea sin nanosegundos
        builder.set_creation_date(Utc::now().with_nanosecond(0).unwrap());
        builder.set_expiration_date(expiration_date);
        builder.set_heartbeat_interval(heartbeat_interval);
        builder.set_notes(notes);
        
    }
}