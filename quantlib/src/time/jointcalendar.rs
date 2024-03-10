use crate::time::calendars::calendar_trait::CalendarTrait;
use crate::time::calendar::Calendar;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use crate::utils::myerror::MyError;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointCalendar {
    name: String,
    calendars: Vec<Calendar>,
}

impl JointCalendar {
    pub fn new(calendars: Vec<Calendar>) -> JointCalendar {
        let mut name = String::from("JoinCalendar : ");
        for (i, cal) in calendars.iter().enumerate() {
            if i == 0 {
                name.push_str(cal.as_trait().calendar_name());
            } else {
            name.push_str(
                format!(
                    "{} & ",
                    cal.as_trait().calendar_name()
                ).as_str())
            }
        }

        JointCalendar { 
            name, 
            calendars,
        }
    }

    pub fn calendars(&self) -> &Vec<Calendar> {
        &self.calendars
    }

    pub fn is_business_day(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().all(|c| c.as_trait().is_business_day(date))
    }                      
}

impl CalendarTrait for JointCalendar {
    fn calendar_name(&self) -> &String {
        &self.name
    }

    fn is_weekend(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().any(|c| c.as_trait().is_weekend(date))
    }

    fn display_holidays(
        &self, 
        date_from: &OffsetDateTime, 
        date_upto: &OffsetDateTime, 
        include_weekend: bool) {
        let mut date = *date_from;
        while date <= *date_upto {
            if self.is_holiday(&date) && (include_weekend || !self.is_weekend(&date)) {
                println!("{:?}", date);
            }
            date = date + time::Duration::days(1);
        }
    }

    fn is_removed_holiday(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().any(|c| c.as_trait().is_removed_holiday(date))
    }

    fn is_added_holiday(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().any(|c| c.as_trait().is_added_holiday(date))
    }

    fn is_base_holiday(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().any(|c| c.as_trait().is_base_holiday(date))
    }

    fn add_holidays(&mut self, date: &time::Date) -> Result<(), MyError> {
        Err(MyError::CallError {
            file: file!().to_string(), 
            line: line!(), 
            contents: "It is not allowed to add holidays to JointCalendar".to_string(),
        })
    }

    fn remove_holidays(&mut self, date: &time::Date) -> Result<(), MyError> {
        Err(MyError::CallError {
            file: file!().to_string(), 
            line: line!(), 
            contents: "It is not allowed to remove holidays from JointCalendar".to_string(),
        })
    }

    fn is_holiday(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().any(|c| c.as_trait().is_holiday(date))
    }

}
         
#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::calendars::unitedstates::{UnitedStates, UnitedStatesType};
    use crate::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
    use crate::time::calendar::{
        Calendar,
        SouthKoreaWrapper,
        UnitedStatesWrapper,
    };
    use time::macros::datetime;
    
    #[test]
    fn test_joint_calendar() {
        let summer_time = false;
        let us = UnitedStates::new(UnitedStatesType::Settlement, summer_time);
        let us_cal = Calendar::UnitedStates(UnitedStatesWrapper{c: us});
        let sk = SouthKorea::new(SouthKoreaType::Settlement);
        let sk_cal = Calendar::SouthKorea(SouthKoreaWrapper{c: sk});

        let joint_calendar = JointCalendar::new(vec![us_cal, sk_cal]);

        let date = datetime!(2021-05-05 00:00:00 +09:00);
        assert_eq!(joint_calendar.is_holiday(&date), true);
        assert_eq!(joint_calendar.is_business_day(&date), false);
    }
}
