use crate::time::calendar_trait::CalendarTrait;
use crate::time::holiday::Holidays;
use serde::{Serialize, Deserialize};
use time::{Date, Month, Weekday, UtcOffset, OffsetDateTime};
use anyhow::Result;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UnitedStatesType {
    Settlement,
    Nyse,
    GovernmentBond,
    FederalReserve,
    Sofr,
}

impl UnitedStatesType {
    fn unpack(&self, date: &OffsetDateTime) -> (i32, Month, u8, Weekday, u16) {
        let year = date.year();
        let month = date.month();
        let day = date.day();
        let weekday = date.weekday();
        let day_of_year = date.ordinal();

        (year, month, day, weekday, day_of_year)
    }

    fn is_washington_birthday(&self, date: &OffsetDateTime) -> bool {
        let (y, m, d, w, _) = self.unpack(&date);
        
        if y >= 1971 {
            (d >= 15 && d <= 21) && w == Weekday::Monday && m == Month::February
        } else {
            (d == 22 || (d == 23 && w == Weekday::Monday) || (d == 21 && w == Weekday::Friday)) && m == Month::February
        }
    }

    fn is_memorial_day(&self, date: &OffsetDateTime) -> bool {
        let (y, m, d, w, _) = self.unpack(&date);

        if y >= 1971 {
            d >= 25 && w == Weekday::Monday && m == Month::May
        } else {
            (d == 30 || (d == 31 && w == Weekday::Monday) || (d == 29 && w == Weekday::Friday)) && m == Month::May
        }
    }

    fn is_labor_day(&self, date: &OffsetDateTime) -> bool {
        let (_, m, d, w, _) = self.unpack(&date);

        d <= 7 && w == Weekday::Monday && m == Month::September
    }

    fn is_columbus_day(&self, date: &OffsetDateTime) -> bool {
        let (y, m, d, w, _) = self.unpack(&date);
        
        (d >= 8 && d <= 14) && w == Weekday::Monday && m == Month::October && y >= 1971
    }

    fn is_veterans_day_no_saturday(&self, date: &OffsetDateTime) -> bool {
        let (y, m, d, w, _) = self.unpack(&date);

        if y <= 1970 || y >= 1978 {
            (d == 11 || (d == 12 && w == Weekday::Monday)) && m == Month::November
        } else {
            (d >= 22 && d <= 28) && w == Weekday::Monday && m == Month::October
        }
    }

    // move_to_friday: false only if the UnitedStatesType is FederalReserve
    fn is_juneteenth(&self, date: &OffsetDateTime, move_to_friday: bool) -> bool {
        // declared in 2021, but only observed by exchanges since 2022
        let (y, m, d, w, _) = self.unpack(&date);
        let res = (d == 19 || (d == 20 && w == Weekday::Monday) 
                        || ((d == 18 && w == Weekday::Friday) && move_to_friday))
                        && (m == Month::June && y >= 2022);
        res
    }

    fn goverment_bond_holiday(&self, date: &OffsetDateTime) -> bool {
        let (y, m, d, w, _) = self.unpack(&date);

        if ((d == 1 || (d == 2 && w == Weekday::Monday)) && m == Month::January) // New Year's Day (possibly moved to Monday if on Sunday)
        || ((d >= 15 && d <= 21) && w == Weekday::Monday && m == Month::January && y >= 1983)// Martin Luther King's birthday (third Monday in January)
        || self.is_washington_birthday(&date)// Washington's birthday (third Monday in February)
        // Good Friday (2015, 2021, 2023 are half day due to NFP/SIFMA; see <https://www.sifma.org/resources/general/holiday-schedule/>)
        || (self.is_good_friday(&date, false) && !(y == 2015 || y == 2021 || y == 2023))
        || self.is_memorial_day(&date) // Memorial Day (last Monday in May)
        || self.is_juneteenth(&date, true) // Juneteenth (Monday if Sunday or Friday if Saturday)
        || ((d == 4 || (d == 5 && w == Weekday::Monday) || (d == 3 && w == Weekday::Friday)) && m == Month::July) // Independence Day (Monday if Sunday or Friday if Saturday)
        || self.is_labor_day(&date)// Labor Day (first Monday in September)
        || self.is_columbus_day(&date)// Columbus Day (second Monday in October)
        || self.is_veterans_day_no_saturday(&date)// Veteran's Day (Monday if Sunday)
        || ((d >= 22 && d <= 28) && w == Weekday::Thursday && m == Month::November) //Thanksgiving Day (fourth Thursday in November)
        || ((d == 25 || (d == 26 && w == Weekday::Monday) || (d == 24 && w == Weekday::Friday)) && m == Month::December)// Christmas (Monday if Sunday or Friday if Saturday)
        {
            return true
        } 
        // special closings
        if (y == 2018 && m == Month::December && d == 5) // President Bush's Funeral
        || (y == 2012 && m == Month::October && d == 30) // Hurricane Sandy
        || (y == 2004 && m == Month::June && d == 11) // President Reagan's funeral
        {
            return true
        }
        return false
    }
}

impl Holidays for UnitedStatesType {
    #[allow(unused_variables)]
    fn is_temporary_holiday(&self, date: &OffsetDateTime) -> bool {
        return false
    }

    fn is_holiday(&self, date: &OffsetDateTime) -> bool {
        // united states holidays are very much hard coded since there are many calendar types: nyse, settlement, government bond, federal reserve
        if self.is_temporary_holiday(&date) {
            return true
        }

        match self {
            UnitedStatesType::Settlement => {
                let (y, m, d, w, _) = self.unpack(&date);

                if ((d == 1 || (d == 2 && w == Weekday::Monday)) && m == Month::January)// New Year's Day (possibly moved to Monday if on Sunday)
                || (d == 31 && w == Weekday::Friday && m == Month::December)// (or to Friday if on Saturday)
                || ((d >= 15 && d <= 21) && w == Weekday::Monday && m == Month::January && y >= 1983) // Martin Luther King's birthday (third Monday in January)
                || self.is_washington_birthday(&date) // Washington's birthday (third Monday in February)
                || self.is_memorial_day(&date) // Memorial Day (last Monday in May)
                || self.is_juneteenth(&date, true)// Juneteenth (Monday if Sunday or Friday if Saturday)
                || ((d == 4 || (d == 5 && w == Weekday::Monday) || (d == 3 && w == Weekday::Friday)) && m == Month::July)// Independence Day (Monday if Sunday or Friday if Saturday)
                || self.is_labor_day(&date)// Labor Day (first Monday in September)
                || self.is_columbus_day(&date)// Columbus Day (second Monday in October)
                || self.is_veterans_day_no_saturday(&date)// Veteran's Day (Monday if Sunday or Friday if Saturday)
                || ((d >= 22 && d <= 28) && w == Weekday::Thursday && m == Month::November)// Thanksgiving Day (fourth Thursday in November)
                || ((d == 25 || (d == 26 && w == Weekday::Monday) || (d == 24 && w == Weekday::Friday)) && m == Month::December)// Christmas (Monday if Sunday or Friday if Saturday)
                {
                    return true 
                }
                return false
            },
            UnitedStatesType::Nyse => {
                let (y, m, d, w, dd) = self.unpack(&date);
                if ((d == 1 || (d == 2 && w == Weekday::Monday)) && m == Month::January) // New Year's Day (possibly moved to Monday if on Sunday)
                || self.is_washington_birthday(&date) // Washington's birthday (third Monday in February)
                || self.is_good_friday(&date, false) // Good Friday
                || self.is_memorial_day(&date) // Memorial Day (last Monday in May)
                || self.is_juneteenth(&date, true) // Juneteenth (Monday if Sunday or Friday if Saturday)
                || ((d == 4 || (d == 5 && w == Weekday::Monday) || (d == 3 && w == Weekday::Friday)) && m == Month::July)// Independence Day (Monday if Sunday or Friday if Saturday)
                || self.is_labor_day(&date) // Labor Day (first Monday in September)
                || ((d >= 22 && d <= 28) && w == Weekday::Thursday && m == Month::November)// Thanksgiving Day (fourth Thursday in November)
                || ((d == 25 || (d == 26 && w == Weekday::Monday) || (d == 24 && w == Weekday::Friday)) && m == Month::December)// Christmas (Monday if Sunday or Friday if Saturday)
                {
                    return true
                }
                if (y >= 1998 && (d >= 15 && d <= 21) && w == Weekday::Monday && m == Month::January) // Martin Luther King's birthday (third Monday in January)
                || ((y <= 1968 || (y <= 1980 && y % 4 == 0)) && m == Month::November && d <= 7 && w == Weekday::Tuesday) // Presidential election days
                {
                    return true
                }
                // Special closings
                if (y == 2018 && m == Month::December && d == 5) // President Bush's Funeral
                || (y == 2012 && m == Month::October && (d == 29 || d == 30)) // Hurricane Sandy
                || (y == 2007 && m == Month::January && d == 2) // President Ford's funeral
                || (y == 2004 && m == Month::June && d == 11) // President Reagan's funeral
                || (y == 2001 && m == Month::September && (11 <= d && d <= 14)) // September 11-14, 2001
                || (y == 1994 && m == Month::April && d == 27) // President Nixon's funeral
                || (y == 1985 && m == Month::September && d == 27) // Hurricane Gloria
                || (y == 1977 && m == Month::July && d == 14) // 1977 Blackout
                || (y == 1973 && m == Month::January && d == 25) // Funeral of former President Lyndon B. Johnson.
                || (y == 1972 && m == Month::December && d == 28) // Funeral of former President Harry S. Truman
                || (y == 1969 && m == Month::July && d == 21) // National Day of Participation for the lunar exploration.
                || (y == 1969 && m == Month::March && d == 31) // Funeral of former President Eisenhower.
                || (y == 1969 && m == Month::February && d == 10) // Closed all day - heavy snow.
                || (y == 1968 && m == Month::July && d == 5) // Day after Independence Day.
                || (y == 1968 && dd >= 163 && w == Weekday::Wednesday) // Four day week (closed on Wednesdays) - Paperwork Crisis
                || (y == 1968 && m == Month::April && d == 9) // Day of mourning for Martin Luther King Jr.
                || (y == 1963 && m == Month::November && d == 25) // Funeral of President Kennedy
                || (y == 1961 && m == Month::May && d == 29) // Day before Decoration Day
                || (y == 1958 && m == Month::December && d == 26) // Day after Christmas
                || ((y == 1954 || y == 1956 || y == 1965) && m == Month::December && d == 24) // Christmas Eve
                {
                    return true
                }
                return false
            },
            UnitedStatesType::GovernmentBond => {
                return self.goverment_bond_holiday(&date)
            },
            UnitedStatesType::Sofr => {
                // so far (that is, up to 2023 at the time of this change) SOFR never fixed
                // on Good Friday.  We're extrapolating that pattern.  This might change if
                // a fixing on Good Friday occurs in future years.
                if self.is_good_friday(&date, false) {
                    return true
                }
                // otherwise it follows the same as UnitedStatesType::GovernmentBond
                return self.goverment_bond_holiday(&date)
            }
            UnitedStatesType::FederalReserve => {
                let (y, m, d, w, _) = self.unpack(&date);
                if ((d == 1 || (d == 2 && w == Weekday::Monday)) && m == Month::January) // New Year's Day (possibly moved to Monday if on Sunday)                   
                    || ((d >= 15 && d <= 21) && w == Weekday::Monday && m == Month::January && y >= 1983) // Martin Luther King's birthday (third Monday in January)
                    || self.is_washington_birthday(&date) // Washington's birthday (third Monday in February)
                    || self.is_memorial_day(&date) // Memorial Day (last Monday in May)
                    || self.is_juneteenth(&date, false) // Juneteenth (Monday if Sunday)
                    || ((d == 4 || (d == 5 && w == Weekday::Monday)) && m == Month::July) // Independence Day (Monday if Sunday)
                    || self.is_labor_day(&date) // Labor Day (first Monday in September)
                    || self.is_columbus_day(&date) // Columbus Day (second Monday in October)
                    || self.is_veterans_day_no_saturday(&date) // Veteran's Day (Monday if Sunday)
                    || ((d >= 22 && d <= 28) && w == Weekday::Thursday && m == Month::November) // Thanksgiving Day (fourth Thursday in November)
                    || ((d == 25 || (d == 26 && w == Weekday::Monday)) && m == Month::December) // Christmas (Monday if Sunday)
                    {
                        return true
                    }
                return false;
            }
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitedStates {
    name: String,
    utc_offset: UtcOffset,
    specific_type: UnitedStatesType,
    holiday_adder: Vec<Date>,
    holiday_remover: Vec<Date>,
}


impl UnitedStates {
    pub fn new(specific_type: UnitedStatesType) -> Self {
        let name = format!("United States ({:?})", specific_type);

        let utc_offset = UtcOffset::from_hms(-5, 0, 0).unwrap();

        let holiday_adder = vec![];
        let holiday_remover = vec![];
        UnitedStates {
            name,
            utc_offset,
            specific_type,
            holiday_adder,
            holiday_remover,
        }
    }
}

impl CalendarTrait for UnitedStates {
    fn calendar_name(&self) -> &String {
        &self.name
    }   
    
    fn add_holidays(&mut self, date: &Date) -> Result<()>{
        self.holiday_adder.push(*date);
        Ok(())
    }

    fn remove_holidays(&mut self, date: &Date) -> Result<()> {
        self.holiday_remover.push(*date);
        Ok(())
    }

    fn is_removed_holiday(&self, date: &OffsetDateTime) -> bool {
        let date = date.date();
        self.holiday_remover.contains(&date)
    }

    fn is_added_holiday(&self, date: &OffsetDateTime) -> bool {
        let date = date.date();
        self.holiday_adder.contains(&date)
    }

    fn is_base_holiday(&self, date: &OffsetDateTime) -> bool {
        self.specific_type.is_holiday(&date)
    }

    fn is_holiday(&self, date: &OffsetDateTime) -> bool {
        let date = date.to_offset(self.utc_offset);
        
        self._is_holiday(&date)
    }

    fn is_weekend(&self, date: &OffsetDateTime) -> bool {
        let date = date.to_offset(self.utc_offset);
        self._is_weekend(&date)
    }

    fn display_holidays(&self,
                        start_date: &OffsetDateTime,
                        end_date: &OffsetDateTime,
                        include_weekend: bool) {
        let start_date = start_date.to_offset(self.utc_offset);
        let end_date = end_date.to_offset(self.utc_offset);

        self._display_holidays( &start_date, &end_date, include_weekend);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_united_states_name() {
        let us = UnitedStates::new(UnitedStatesType::Settlement, false);
        assert_eq!(us.calendar_name(), "United States (Settlement)");
        
        let us = UnitedStates::new(UnitedStatesType::Nyse, false);
        assert_eq!(us.calendar_name(), "United States (Nyse)");

        let us = UnitedStates::new(UnitedStatesType::GovernmentBond, false);
        assert_eq!(us.calendar_name(), "United States (GovernmentBond)");
    }

    #[test]
    fn test_united_states_holidays() {
        use time::macros::datetime;
        
        let us = UnitedStates::new(UnitedStatesType::Settlement, false);
        let date = datetime!(2022-1-1 00:00:00 -5:00);

        assert_eq!(us.is_base_holiday(&date), true);
        assert_eq!(us.is_removed_holiday(&date), false);
        assert_eq!(us.is_added_holiday(&date), false);
    }

}
