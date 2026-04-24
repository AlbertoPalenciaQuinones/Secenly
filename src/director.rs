use crate::builder::Builder;
use chrono::{Utc, Duration};
pub struct Director;

impl Director {
    
    pub fn construct_license(builder: &mut impl Builder, id: [u8; 64]) {
        let now = Utc::now();

        builder.set_id(id);
        builder.set_creation_date(now);
        builder.set_last_use_date(now);
        builder.set_expiration_date(now + Duration::days(30));
        builder.set_heartbeat_interval(60);
        builder.set_notes("Objeto creado por defecto".to_string());
    }

}