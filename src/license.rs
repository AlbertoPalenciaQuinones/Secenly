use chrono::{DateTime, Utc};
use rasn::{AsnType, Encode, Decode};

#[derive(AsnType, Encode, Decode, Debug)]
pub struct License {
    
    pub id: [u8; 64],
    pub creation_date: DateTime<Utc>,
    pub expiration_date: DateTime<Utc>,
    pub last_use_date: DateTime<Utc>,
    pub heartbeat_interval: i32,
    pub notes: String,
}

impl License {
    pub fn new(
        id: [u8; 64],
        creation_date: DateTime<Utc>,
        expiration_date: DateTime<Utc>,
        last_use_date: DateTime<Utc>,
        heartbeat_interval: i32,
        notes: String, 
    ) -> License {
        License {
            id,
            creation_date,
            expiration_date,
            last_use_date: last_use_date,
            heartbeat_interval: heartbeat_interval,
            notes,
        }
    }

    pub fn set_id(&mut self, id: [u8; 64]) {
        self.id = id;
    }


    pub fn get_id(&self) -> &[u8; 64] {
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
    }
}
