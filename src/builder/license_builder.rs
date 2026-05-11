use chrono::{DateTime, Utc};

use crate::{builder::builder::Builder, domain::license::License};

// (Default) proporciona una inicialización segura del builder, garantizando que
// la construcción comience desde un estado conocido.
#[derive(Default)]
// Contenedor de estado parcial
pub struct LicenseBuilder {
    id: Option<String>,
    creation_date: Option<DateTime<Utc>>,
    expiration_date: Option<DateTime<Utc>>,
    heartbeat_interval: Option<i32>,
    notes: Option<String>,
}

// Builder encargado de mantener el estado intermedio durante la construcción
// de una licencia. Se puede configurar de forma incremental los campos de ella.
impl Builder for LicenseBuilder {
    type OutputType = License;

    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    fn set_creation_date(&mut self, creation_date: DateTime<Utc>) {
        self.creation_date = Some(creation_date);
    }

    fn set_expiration_date(&mut self, expiration_date: DateTime<Utc>) {
        self.expiration_date = Some(expiration_date);
    }

    fn set_heartbeat_interval(&mut self, heartbeat_interval: i32) {
        self.heartbeat_interval = Some(heartbeat_interval);
    }

    fn set_notes(&mut self, notes: String) {
        self.notes = Some(notes);
    }

    fn build(self) -> License {
        License::new(
            self.id.expect("License id must be set"),
            self.creation_date.expect("Creation date must be set"),
            self.expiration_date.expect("Expiration date must be set"),
            self.heartbeat_interval.expect("Heartbeat interval must be set"),
            self.notes.expect("License notes must be set"),
        )
    }
}
