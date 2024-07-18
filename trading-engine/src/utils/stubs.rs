use crate::types::venue::Venue;

pub fn get_utc_hour(venue: Venue) -> u8 {
    match venue {
        Venue::KRX => 9,
        _ => 0,
    }
}