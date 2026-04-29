use crate::builder::Builder;

use chrono::{DateTime, Timelike, Utc};
pub struct Director;

impl Director {
    
    pub fn construct_license(
        builder: &mut impl Builder, 
        id: [u8; 64],
        expiration_date: DateTime<Utc>,
        heartbeat_interval: i32,
        notes: String,
        ) {

        builder.set_id(id);
        builder.set_creation_date(Utc::now().with_nanosecond(0).unwrap());
        builder.set_last_use_date(Utc::now().with_nanosecond(0).unwrap());
        builder.set_expiration_date(expiration_date);
        builder.set_heartbeat_interval(heartbeat_interval);
        builder.set_notes(notes);
    }
}