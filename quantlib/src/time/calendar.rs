use enum_dispatch;
use crate::time::calendars::calendar_trait::CalendarTrait;
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

