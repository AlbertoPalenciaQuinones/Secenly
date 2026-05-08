use crate::license::License;
use crate::builder::Builder;
use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct LicenseBuilder {
    id: Option<[u8; 64]>,
    creation_date: Option<DateTime<Utc>>,
    expiration_date: Option<DateTime<Utc>>,
    last_use_date: Option<DateTime<Utc>>,
    heartbeat_interval: Option<i32>,
    notes: Option<String>,
}

impl Builder for LicenseBuilder {
    type OutputType = License;

    fn set_id(&mut self, id: [u8; 64]) {
        self.id = Some(id);
    }

    fn set_creation_date(&mut self, creation_date: DateTime<Utc>) {
        self.creation_date = Some(creation_date);
    }

    fn set_expiration_date(&mut self, expiration_date: DateTime<Utc>) {
        self.expiration_date = Some(expiration_date);
    }

    fn set_last_use_date(&mut self, last_use_date: DateTime<Utc>) {
        self.last_use_date = Some(last_use_date);
    }

    fn set_heartbeat_interval(&mut self, heartbeat_interval: i32) {
        self.heartbeat_interval = Some(heartbeat_interval);

    }

    fn set_notes(&mut self, notes: String) {
        self.notes = Some(notes);
    }

    fn build(self) -> License {
        License::new(
            self.id.expect("Please, set a car type"),
            self.creation_date.expect("Please, set a number of seats"),
            self.expiration_date.expect("Please, set an engine configuration"),
            self.last_use_date.expect("Please, set up transmission"),
            self.heartbeat_interval.expect("Please, set heartbeat interval"),

            self.notes.expect("Establecer la nota de la licencia"),
        )
    }
}
