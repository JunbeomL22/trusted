use crate::time::calendars::{
    nullcalendar::NullCalendar, southkorea::SouthKorea, unitedstates::UnitedStates,
};
use enum_dispatch;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[enum_dispatch::enum_dispatch(CalendarTrait)]
pub enum Calendar {
    NullCalendar(NullCalendar),
    SouthKorea(SouthKorea),
    UnitedStates(UnitedStates),
}

impl Default for Calendar {
    fn default() -> Self {
        Calendar::NullCalendar(NullCalendar::default())
    }
}
