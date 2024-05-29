use enum_dispatch;
use serde::{Serialize, Deserialize};
use crate::time::calendars::{
    southkorea::SouthKorea,
    unitedstates::UnitedStates,
    nullcalendar::NullCalendar,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

