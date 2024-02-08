use crate::time::calendar::Calendar;
use time::OffsetDateTime;

pub struct JointCalendar {
    pub calendars: Vec<Box<dyn Calendar>>,
}

impl JointCalendar {
    pub fn new(calendars: Vec<Box<dyn Calendar>>) -> JointCalendar {
        JointCalendar { calendars }
    }

    pub fn calendars(&self) -> &Vec<Box<dyn Calendar>> {
        &self.calendars
    }

    pub fn is_holiday(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().any(|c| c.is_holiday(date))
    }

    pub fn is_business_day(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().all(|c| c.is_business_day(date))
    }

    fn is_weekend(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().any(|c| c.is_weekend(date))
    }
    
    pub fn display_holidays(&self, 
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
}
                    
#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::calendars::unitedstates::{UnitedStates, UnitedStatesType};
    use crate::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
    use time::macros::datetime;
    
    #[test]
    fn test_joint_calendar() {
        let summer_time = false;
        let us = UnitedStates::new(UnitedStatesType::Settlement, summer_time);
        let sk = SouthKorea::new(SouthKoreaType::Settlement);
        let joint_calendar = JointCalendar::new(vec![Box::new(us), Box::new(sk)]);

        let date = datetime!(2021-05-05 00:00:00 +09:00);
        assert_eq!(joint_calendar.is_holiday(&date), true);
        assert_eq!(joint_calendar.is_business_day(&date), false);
    }
}
