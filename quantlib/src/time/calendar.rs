use crate::time::calendars::calendar_trait::CalendarTrait;
use serde::{Serialize, Deserialize};
use crate::time::calendars::{
    southkorea::SouthKorea,
    unitedstates::UnitedStates,
    nullcalendar::NullCalendar,
};

#[derive(Serialize, Deserialize, Debug, Clone)] pub struct NullCalendarWrapper{pub c: NullCalendar}

#[derive(Serialize, Deserialize, Debug, Clone)] pub struct SouthKoreaWrapper{pub c: SouthKorea}

#[derive(Serialize, Deserialize, Debug, Clone)] pub struct UnitedStatesWrapper{pub c: UnitedStates}
//
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Calendar {
    NullCalendar(NullCalendarWrapper),
    SouthKorea(SouthKoreaWrapper),
    UnitedStates(UnitedStatesWrapper),
}

impl Calendar {
    pub fn as_trait(&self) -> &(dyn CalendarTrait) {
        match self {
            Calendar::NullCalendar(NullCalendarWrapper{c: cal}) => &*cal,
            Calendar::SouthKorea(SouthKoreaWrapper{c: cal}) => &*cal,
            Calendar::UnitedStates(UnitedStatesWrapper{c: cal}) => &*cal,
        }
    }
}