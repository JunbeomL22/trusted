use crate::definitions::Time;
use crate::time::calendar::Calendar;
use crate::time::calendar_trait::CalendarTrait;
use crate::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JointCalendar {
    name: String,
    calendars: Vec<Calendar>,
}

impl Default for JointCalendar {
    fn default() -> JointCalendar {
        let sk = SouthKorea::new(SouthKoreaType::Settlement);
        let sk_cal = Calendar::SouthKorea(sk);
        JointCalendar {
            name: sk.calendar_name().clone(),
            calendars: vec![sk_cal],
        }
    }
}

impl JointCalendar {
    pub fn new(calendars: Vec<Calendar>) -> Result<JointCalendar> {
        if calendars.is_empty() {
            return Err(anyhow!(
                "{}:{}  JointCalendar must have at least one calendar",
                file!(),
                line!()
            ));
        }
        let mut name = String::from("JoinCalendar : ");
        for (i, cal) in calendars.iter().enumerate() {
            if i == 0 {
                name.push_str(cal.calendar_name());
            } else {
                name.push_str(format!("{} & ", cal.calendar_name()).as_str())
            }
        }

        Ok(JointCalendar { name, calendars })
    }

    pub fn calendars(&self) -> &Vec<Calendar> {
        &self.calendars
    }

    pub fn is_business_day(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().all(|c| c.is_business_day(date))
    }
}

impl CalendarTrait for JointCalendar {
    fn year_fraction(
        &self,
        start_date: &OffsetDateTime,
        end_date: &OffsetDateTime,
        day_count: &super::conventions::DayCountConvention,
    ) -> Result<Time> {
        self.calendars[0].year_fraction(start_date, end_date, day_count)
    }

    fn calendar_name(&self) -> &String {
        &self.name
    }

    fn is_weekend(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().any(|c| c.is_weekend(date))
    }

    fn display_holidays(
        &self,
        date_from: &OffsetDateTime,
        date_upto: &OffsetDateTime,
        include_weekend: bool,
    ) {
        let mut date = *date_from;
        while date <= *date_upto {
            if self.is_holiday(&date) && (include_weekend || !self.is_weekend(&date)) {
                println!("{:?}", date);
            }
            date += time::Duration::days(1);
        }
    }

    fn is_removed_holiday(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().any(|c| c.is_removed_holiday(date))
    }

    fn is_added_holiday(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().any(|c| c.is_added_holiday(date))
    }

    fn is_base_holiday(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().any(|c| c.is_base_holiday(date))
    }

    fn add_holidays(&mut self, _date: &time::Date) -> Result<()> {
        Err(anyhow!(
            "It is not allowed to add holidays to JointCalendar"
        ))
    }

    fn remove_holidays(&mut self, _date: &time::Date) -> Result<()> {
        Err(anyhow!(
            "It is not allowed to remove holidays from JointCalendar"
        ))
    }

    fn is_holiday(&self, date: &OffsetDateTime) -> bool {
        self.calendars.iter().any(|c| c.is_holiday(date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::calendar::Calendar;
    use crate::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
    use crate::time::calendars::unitedstates::{UnitedStates, UnitedStatesType};
    use time::macros::datetime;

    #[test]
    fn test_joint_calendar() -> Result<()> {
        let _summer_time = false;
        let us = UnitedStates::new(UnitedStatesType::Settlement);
        let us_cal = Calendar::UnitedStates(us);
        let sk = SouthKorea::new(SouthKoreaType::Settlement);
        let sk_cal = Calendar::SouthKorea(sk);

        let joint_calendar = JointCalendar::new(vec![us_cal, sk_cal])?;

        let date = datetime!(2021-05-05 00:00:00 +09:00);
        assert_eq!(joint_calendar.is_holiday(&date), true);
        assert_eq!(joint_calendar.is_business_day(&date), false);
        Ok(())
    }
}
