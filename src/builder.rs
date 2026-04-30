use chrono::{DateTime, Utc};

pub trait Builder {
    type OutputType;

    fn set_id(&mut self, id: String);
    fn set_creation_date(&mut self, creation_date: DateTime<Utc>);
    fn set_expiration_date(&mut self, expiration_date: DateTime<Utc>);
    fn set_last_use_date(&mut self, last_use_date: DateTime<Utc>);
    fn set_heartbeat_interval(&mut self, heartbeat_interval: i32);
    fn set_notes(&mut self, notes: String);

    fn build(self) -> Self::OutputType;
}