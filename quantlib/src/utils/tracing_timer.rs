use time::{format_description::well_known::Rfc3339, OffsetDateTime, UtcOffset};
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::format::Writer;

#[derive(Debug, Clone)]
pub struct CustomOffsetTime {
    offset: UtcOffset,
}

impl CustomOffsetTime {
    pub fn new(hours: i8, minutes: i8, seconds: i8) -> Self {
        let offset = UtcOffset::from_hms(hours, minutes, seconds)
            .expect("Invalid offset");
        Self { offset }
    }
}

impl FormatTime for CustomOffsetTime {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let now = OffsetDateTime::now_utc().to_offset(self.offset);
        let timestamp = now.format(&Rfc3339).unwrap();
        write!(w, "{}", timestamp)
    }
}