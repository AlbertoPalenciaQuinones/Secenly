use chrono::{DateTime, Utc};

// Encapsula la construcción paso a paso de una licencia, permitiendo su 
// configuración (eliminación o adición de atributos)
pub trait Builder {
    // OutputType permite que se trabaje con distintos tipos de licencias
    type OutputType;

    // Los estados son mutables (al igual que en el director, se modifican)
    fn set_id(&mut self, id: String);
    fn set_creation_date(&mut self, creation_date: DateTime<Utc>);
    fn set_expiration_date(&mut self, expiration_date: DateTime<Utc>);
    fn set_heartbeat_interval(&mut self, heartbeat_interval: i32);
    fn set_notes(&mut self, notes: String);

    fn build(self) -> Self::OutputType;
}