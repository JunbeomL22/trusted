use crate::time::calendars::calendar_trait::CalendarTrait;
use crate::time::holiday::Holidays;
use crate::time::constants::{KOREAN_LUNAR_NEWYEARS, FIRST_LUNAR_NEWYEAR, LAST_LUNAR_NEWYEAR};
use anyhow::Result;
use time::{Date, Duration, Month, OffsetDateTime, UtcOffset};
use log::warn;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum SouthKoreaType {
    Krx,
    Settlement,
}

impl SouthKoreaType {
    fn is_lunar_new_year(&self, date: &OffsetDateTime) -> bool {
        let date = date.date();
        let year = date.year();

        if (year as usize > LAST_LUNAR_NEWYEAR) || (FIRST_LUNAR_NEWYEAR > year as usize) {
            warn!("Lunar New Year is not available for the year {}", year);
            return false;
        }

        let solar_ordinal = KOREAN_LUNAR_NEWYEARS[year as usize - FIRST_LUNAR_NEWYEAR as usize];

        let lunar_new_year_day = Date::from_ordinal_date(year, solar_ordinal as u16).unwrap();

        if (lunar_new_year_day == date)
            || (lunar_new_year_day + Duration::days(1) == date)
            || (lunar_new_year_day - Duration::days(1) == date) {
            return true;
        }
        return false;
    }

    fn is_labour_day(&self, date: &OffsetDateTime) -> bool{
        let month = date.month();
        let day = date.day();

        if month == Month::May && day == 1 {
            return true;
        }
        return false;
    }

    fn is_buddha_birthday(&self, date: &OffsetDateTime) -> bool {
        let date = date.date();
        let year = date.year();
        let month = date.month();
        let day = date.day();

        if (year < 1951) || (year > 2050) {
            warn!("Buddha's Birthday is not implemented for the year {}", year);
            return false;
        }
        
        let res = match (year, month, day) {
            (1951, Month::May, 13) => true,
            (1952, Month::May, 1) => true,
            (1953, Month::May, 20) => true,
            (1954, Month::May, 10) => true,
            (1955, Month::May, 29) => true,
            (1956, Month::May, 17) => true,
            (1957, Month::May, 7) => true,
            (1958, Month::May, 26) => true,
            (1959, Month::May, 15) => true,
            (1960, Month::May, 3) => true,
            (1961, Month::May, 22) => true,
            (1962, Month::May, 11) => true,
            (1963, Month::May, 1) => true,
            (1964, Month::May, 19) => true,
            (1965, Month::May, 8) => true,
            (1966, Month::May, 27) => true,
            (1967, Month::May, 16) => true,
            (1968, Month::May, 5) => true,
            (1969, Month::May, 23) => true,
            (1970, Month::May, 12) => true,
            (1971, Month::May, 2) => true,
            (1972, Month::May, 20) => true,
            (1973, Month::May, 10) => true,
            (1974, Month::April, 29) => true,
            (1975, Month::May, 18) => true,
            (1976, Month::May, 6) => true,
            (1977, Month::May, 25) => true,
            (1978, Month::May, 14) => true,
            (1979, Month::May, 3) => true,
            (1980, Month::May, 21) => true,
            (1981, Month::May, 11) => true,
            (1982, Month::May, 1) => true,
            (1983, Month::May, 20) => true,
            (1984, Month::May, 8) => true,
            (1985, Month::May, 27) => true,
            (1986, Month::May, 16) => true,
            (1987, Month::May, 5) => true,
            (1988, Month::May, 23) => true,
            (1989, Month::May, 12) => true,
            (1990, Month::May, 2) => true,
            (1991, Month::May, 21) => true,
            (1992, Month::May, 10) => true,
            (1993, Month::May, 28) => true,
            (1994, Month::May, 18) => true,
            (1995, Month::May, 7) => true,
            (1996, Month::May, 24) => true,
            (1997, Month::May, 14) => true,
            (1998, Month::May, 3) => true,
            (1999, Month::May, 22) => true,
            (2000, Month::May, 11) => true,
            (2001, Month::May, 1) => true,
            (2002, Month::May, 19) => true,
            (2003, Month::May, 8) => true,
            (2004, Month::May, 26) => true,
            (2005, Month::May, 15) => true,
            (2006, Month::May, 5) => true,
            (2007, Month::May, 24) => true,
            (2008, Month::May, 12) => true,
            (2009, Month::May, 2) => true,
            (2010, Month::May, 21) => true,
            (2011, Month::May, 10) => true,
            (2012, Month::May, 28) => true,
            (2013, Month::May, 17) => true,
            (2014, Month::May, 6) => true,
            (2015, Month::May, 25) => true,
            (2016, Month::May, 14) => true,
            (2017, Month::May, 3) => true,
            (2018, Month::May, 22) => true,
            (2019, Month::May, 12) => true,
            (2020, Month::April, 30) => true,
            (2021, Month::May, 19) => true,
            (2022, Month::May, 8) => true,
            (2023, Month::May, 27) => true,
            (2024, Month::May, 15) => true,
            (2025, Month::May, 5) => true,
            (2026, Month::May, 24) => true,
            (2027, Month::May, 13) => true,
            (2028, Month::May, 2) => true,
            (2029, Month::May, 20) => true,
            (2030, Month::May, 9) => true,
            (2031, Month::May, 28) => true,
            (2032, Month::May, 16) => true,
            (2033, Month::May, 6) => true,
            (2034, Month::May, 25) => true,
            (2035, Month::May, 15) => true,
            (2036, Month::May, 3) => true,
            (2037, Month::May, 22) => true,
            (2038, Month::May, 11) => true,
            (2039, Month::April, 30) => true,
            (2040, Month::May, 18) => true,
            (2041, Month::May, 7) => true,
            (2042, Month::May, 26) => true,
            (2043, Month::May, 16) => true,
            (2044, Month::May, 5) => true,
            (2045, Month::May, 24) => true,
            (2046, Month::May, 13) => true,
            (2047, Month::May, 2) => true,
            (2048, Month::May, 20) => true,
            (2049, Month::May, 9) => true,
            (2050, Month::May, 28) => true,
            _ => false,
        };
        res
    }

    fn is_chuseok(&self, date: &OffsetDateTime) -> bool{
        let year = date.year();
        let month = date.month();
        let day = date.day();
        
        if (year < 1951) || (year > 2050) {
            warn!("Chuseok is not implemented for the year {}", year);
            return false;
        }

        let res = match (year, month, day) {
            (1951, Month::September, 14) => true,
            (1951, Month::September, 15) => true,
            (1951, Month::September, 16) => true,
            (1952, Month::October, 2) => true,
            (1952, Month::October, 3) => true,
            (1952, Month::October, 4) => true,
            (1953, Month::September, 21) => true,
            (1953, Month::September, 22) => true,
            (1953, Month::September, 23) => true,
            (1954, Month::September, 10) => true,
            (1954, Month::September, 11) => true,
            (1954, Month::September, 12) => true,
            (1955, Month::September, 29) => true,
            (1955, Month::September, 30) => true,
            (1955, Month::October, 1) => true,
            (1956, Month::September, 18) => true,
            (1956, Month::September, 19) => true,
            (1956, Month::September, 20) => true,
            (1957, Month::September, 7) => true,
            (1957, Month::September, 8) => true,
            (1957, Month::September, 9) => true,
            (1958, Month::September, 26) => true,
            (1958, Month::September, 27) => true,
            (1958, Month::September, 28) => true,
            (1959, Month::September, 16) => true,
            (1959, Month::September, 17) => true,
            (1959, Month::September, 18) => true,
            (1960, Month::October, 4) => true,
            (1960, Month::October, 5) => true,
            (1960, Month::October, 6) => true,
            (1961, Month::September, 23) => true,
            (1961, Month::September, 24) => true,
            (1961, Month::September, 25) => true,
            (1962, Month::September, 12) => true,
            (1962, Month::September, 13) => true,
            (1962, Month::September, 14) => true,
            (1963, Month::October, 1) => true,
            (1963, Month::October, 2) => true,
            (1963, Month::October, 3) => true,
            (1964, Month::September, 19) => true,
            (1964, Month::September, 20) => true,
            (1964, Month::September, 21) => true,
            (1965, Month::September, 9) => true,
            (1965, Month::September, 10) => true,
            (1965, Month::September, 11) => true,
            (1966, Month::September, 28) => true,
            (1966, Month::September, 29) => true,
            (1966, Month::September, 30) => true,
            (1967, Month::September, 17) => true,
            (1967, Month::September, 18) => true,
            (1967, Month::September, 19) => true,
            (1968, Month::October, 5) => true,
            (1968, Month::October, 6) => true,
            (1968, Month::October, 7) => true,
            (1969, Month::September, 25) => true,
            (1969, Month::September, 26) => true,
            (1969, Month::September, 27) => true,
            (1970, Month::September, 14) => true,
            (1970, Month::September, 15) => true,
            (1970, Month::September, 16) => true,
            (1971, Month::October, 2) => true,
            (1971, Month::October, 3) => true,
            (1971, Month::October, 4) => true,
            (1972, Month::September, 21) => true,
            (1972, Month::September, 22) => true,
            (1972, Month::September, 23) => true,
            (1973, Month::September, 10) => true,
            (1973, Month::September, 11) => true,
            (1973, Month::September, 12) => true,
            (1974, Month::September, 29) => true,
            (1974, Month::September, 30) => true,
            (1974, Month::October, 1) => true,
            (1975, Month::September, 19) => true,
            (1975, Month::September, 20) => true,
            (1975, Month::September, 21) => true,
            (1976, Month::September, 7) => true,
            (1976, Month::September, 8) => true,
            (1976, Month::September, 9) => true,
            (1977, Month::September, 26) => true,
            (1977, Month::September, 27) => true,
            (1977, Month::September, 28) => true,
            (1978, Month::September, 16) => true,
            (1978, Month::September, 17) => true,
            (1978, Month::September, 18) => true,
            (1979, Month::October, 4) => true,
            (1979, Month::October, 5) => true,
            (1979, Month::October, 6) => true,
            (1980, Month::September, 22) => true,
            (1980, Month::September, 23) => true,
            (1980, Month::September, 24) => true,
            (1981, Month::September, 11) => true,
            (1981, Month::September, 12) => true,
            (1981, Month::September, 13) => true,
            (1982, Month::September, 30) => true,
            (1982, Month::October, 1) => true,
            (1982, Month::October, 2) => true,
            (1983, Month::September, 20) => true,
            (1983, Month::September, 21) => true,
            (1983, Month::September, 22) => true,
            (1984, Month::September, 9) => true,
            (1984, Month::September, 10) => true,
            (1984, Month::September, 11) => true,
            (1985, Month::September, 28) => true,
            (1985, Month::September, 29) => true,
            (1985, Month::September, 30) => true,
            (1986, Month::September, 17) => true,
            (1986, Month::September, 18) => true,
            (1986, Month::September, 19) => true,
            (1987, Month::October, 6) => true,
            (1987, Month::October, 7) => true,
            (1987, Month::October, 8) => true,
            (1988, Month::September, 24) => true,
            (1988, Month::September, 25) => true,
            (1988, Month::September, 26) => true,
            (1989, Month::September, 13) => true,
            (1989, Month::September, 14) => true,
            (1989, Month::September, 15) => true,
            (1990, Month::October, 2) => true,
            (1990, Month::October, 3) => true,
            (1990, Month::October, 4) => true,
            (1991, Month::September, 21) => true,
            (1991, Month::September, 22) => true,
            (1991, Month::September, 23) => true,
            (1992, Month::September, 10) => true,
            (1992, Month::September, 11) => true,
            (1992, Month::September, 12) => true,
            (1993, Month::September, 29) => true,
            (1993, Month::September, 30) => true,
            (1993, Month::October, 1) => true,
            (1994, Month::September, 19) => true,
            (1994, Month::September, 20) => true,
            (1994, Month::September, 21) => true,
            (1995, Month::September, 8) => true,
            (1995, Month::September, 9) => true,
            (1995, Month::September, 10) => true,
            (1996, Month::September, 26) => true,
            (1996, Month::September, 27) => true,
            (1996, Month::September, 28) => true,
            (1997, Month::September, 15) => true,
            (1997, Month::September, 16) => true,
            (1997, Month::September, 17) => true,
            (1998, Month::October, 4) => true,
            (1998, Month::October, 5) => true,
            (1998, Month::October, 6) => true,
            (1999, Month::September, 23) => true,
            (1999, Month::September, 24) => true,
            (1999, Month::September, 25) => true,
            (2000, Month::September, 11) => true,
            (2000, Month::September, 12) => true,
            (2000, Month::September, 13) => true,
            (2001, Month::September, 30) => true,
            (2001, Month::October, 1) => true,
            (2001, Month::October, 2) => true,
            (2002, Month::September, 20) => true,
            (2002, Month::September, 21) => true,
            (2002, Month::September, 22) => true,
            (2003, Month::September, 10) => true,
            (2003, Month::September, 11) => true,
            (2003, Month::September, 12) => true,
            (2004, Month::September, 27) => true,
            (2004, Month::September, 28) => true,
            (2004, Month::September, 29) => true,
            (2005, Month::September, 17) => true,
            (2005, Month::September, 18) => true,
            (2005, Month::September, 19) => true,
            (2006, Month::October, 5) => true,
            (2006, Month::October, 6) => true,
            (2006, Month::October, 7) => true,
            (2007, Month::September, 24) => true,
            (2007, Month::September, 25) => true,
            (2007, Month::September, 26) => true,
            (2008, Month::September, 13) => true,
            (2008, Month::September, 14) => true,
            (2008, Month::September, 15) => true,
            (2009, Month::October, 2) => true,
            (2009, Month::October, 3) => true,
            (2009, Month::October, 4) => true,
            (2010, Month::September, 21) => true,
            (2010, Month::September, 22) => true,
            (2010, Month::September, 23) => true,
            (2011, Month::September, 11) => true,
            (2011, Month::September, 12) => true,
            (2011, Month::September, 13) => true,
            (2012, Month::September, 29) => true,
            (2012, Month::September, 30) => true,
            (2012, Month::October, 1) => true,
            (2013, Month::September, 18) => true,
            (2013, Month::September, 19) => true,
            (2013, Month::September, 20) => true,
            (2014, Month::September, 7) => true,
            (2014, Month::September, 8) => true,
            (2014, Month::September, 9) => true,
            (2015, Month::September, 26) => true,
            (2015, Month::September, 27) => true,
            (2015, Month::September, 28) => true,
            (2016, Month::September, 14) => true,
            (2016, Month::September, 15) => true,
            (2016, Month::September, 16) => true,
            (2017, Month::October, 3) => true,
            (2017, Month::October, 4) => true,
            (2017, Month::October, 5) => true,
            (2018, Month::September, 23) => true,
            (2018, Month::September, 24) => true,
            (2018, Month::September, 25) => true,
            (2019, Month::September, 12) => true,
            (2019, Month::September, 13) => true,
            (2019, Month::September, 14) => true,
            (2020, Month::September, 30) => true,
            (2020, Month::October, 1) => true,
            (2020, Month::October, 2) => true,
            (2021, Month::September, 20) => true,
            (2021, Month::September, 21) => true,
            (2021, Month::September, 22) => true,
            (2022, Month::September, 9) => true,
            (2022, Month::September, 10) => true,
            (2022, Month::September, 11) => true,
            (2023, Month::September, 28) => true,
            (2023, Month::September, 29) => true,
            (2023, Month::September, 30) => true,
            (2024, Month::September, 16) => true,
            (2024, Month::September, 17) => true,
            (2024, Month::September, 18) => true,
            (2025, Month::October, 5) => true,
            (2025, Month::October, 6) => true,
            (2025, Month::October, 7) => true,
            (2026, Month::September, 24) => true,
            (2026, Month::September, 25) => true,
            (2026, Month::September, 26) => true,
            (2027, Month::September, 14) => true,
            (2027, Month::September, 15) => true,
            (2027, Month::September, 16) => true,
            (2028, Month::October, 2) => true,
            (2028, Month::October, 3) => true,
            (2028, Month::October, 4) => true,
            (2029, Month::September, 21) => true,
            (2029, Month::September, 22) => true,
            (2029, Month::September, 23) => true,
            (2030, Month::September, 11) => true,
            (2030, Month::September, 12) => true,
            (2030, Month::September, 13) => true,
            (2031, Month::September, 30) => true,
            (2031, Month::October, 1) => true,
            (2031, Month::October, 2) => true,
            (2032, Month::September, 18) => true,
            (2032, Month::September, 19) => true,
            (2032, Month::September, 20) => true,
            (2033, Month::September, 7) => true,
            (2033, Month::September, 8) => true,
            (2033, Month::September, 9) => true,
            (2034, Month::September, 26) => true,
            (2034, Month::September, 27) => true,
            (2034, Month::September, 28) => true,
            (2035, Month::September, 15) => true,
            (2035, Month::September, 16) => true,
            (2035, Month::September, 17) => true,
            (2036, Month::October, 3) => true,
            (2036, Month::October, 4) => true,
            (2036, Month::October, 5) => true,
            (2037, Month::September, 23) => true,
            (2037, Month::September, 24) => true,
            (2037, Month::September, 25) => true,
            (2038, Month::September, 12) => true,
            (2038, Month::September, 13) => true,
            (2038, Month::September, 14) => true,
            (2039, Month::October, 1) => true,
            (2039, Month::October, 2) => true,
            (2039, Month::October, 3) => true,
            (2040, Month::September, 20) => true,
            (2040, Month::September, 21) => true,
            (2040, Month::September, 22) => true,
            (2041, Month::September, 9) => true,
            (2041, Month::September, 10) => true,
            (2041, Month::September, 11) => true,
            (2042, Month::September, 27) => true,
            (2042, Month::September, 28) => true,
            (2042, Month::September, 29) => true,
            (2043, Month::September, 16) => true,
            (2043, Month::September, 17) => true,
            (2043, Month::September, 18) => true,
            (2044, Month::October, 4) => true,
            (2044, Month::October, 5) => true,
            (2044, Month::October, 6) => true,
            (2045, Month::September, 24) => true,
            (2045, Month::September, 25) => true,
            (2045, Month::September, 26) => true,
            (2046, Month::September, 14) => true,
            (2046, Month::September, 15) => true,
            (2046, Month::September, 16) => true,
            (2047, Month::October, 3) => true,
            (2047, Month::October, 4) => true,
            (2047, Month::October, 5) => true,
            (2048, Month::September, 21) => true,
            (2048, Month::September, 22) => true,
            (2048, Month::September, 23) => true,
            (2049, Month::September, 10) => true,
            (2049, Month::September, 11) => true,
            (2049, Month::September, 12) => true,
            (2050, Month::September, 29) => true,
            (2050, Month::September, 30) => true,
            (2050, Month::October, 1) => true,
            _ => false,
        };
        res
    }

           
}

impl Holidays for SouthKoreaType {
    fn is_last_business_day_of_year(&self, date: &OffsetDateTime) -> bool{
        // hard coded for avoiding infinite loop
        let year = date.year();
        let month = date.month();
        let day = date.day();

        if (year < 1951) || (year > 2050) {
            warn!("Last business day of the year is not implemented for the year {}", year);
            return false;
        }

        let res = match (year, month, day) {
            (1951, Month::December, 31) => true,
            (1952, Month::December, 31) => true,
            (1953, Month::December, 31) => true,
            (1954, Month::December, 31) => true,
            (1955, Month::December, 30) => true,
            (1956, Month::December, 31) => true,
            (1957, Month::December, 31) => true,
            (1958, Month::December, 31) => true,
            (1959, Month::December, 31) => true,
            (1960, Month::December, 30) => true,
            (1961, Month::December, 29) => true,
            (1962, Month::December, 31) => true,
            (1963, Month::December, 31) => true,
            (1964, Month::December, 31) => true,
            (1965, Month::December, 31) => true,
            (1966, Month::December, 30) => true,
            (1967, Month::December, 29) => true,
            (1968, Month::December, 31) => true,
            (1969, Month::December, 31) => true,
            (1970, Month::December, 31) => true,
            (1971, Month::December, 31) => true,
            (1972, Month::December, 29) => true,
            (1973, Month::December, 31) => true,
            (1974, Month::December, 31) => true,
            (1975, Month::December, 31) => true,
            (1976, Month::December, 31) => true,
            (1977, Month::December, 30) => true,
            (1978, Month::December, 29) => true,
            (1979, Month::December, 31) => true,
            (1980, Month::December, 31) => true,
            (1981, Month::December, 31) => true,
            (1982, Month::December, 31) => true,
            (1983, Month::December, 30) => true,
            (1984, Month::December, 31) => true,
            (1985, Month::December, 31) => true,
            (1986, Month::December, 31) => true,
            (1987, Month::December, 31) => true,
            (1988, Month::December, 30) => true,
            (1989, Month::December, 29) => true,
            (1990, Month::December, 31) => true,
            (1991, Month::December, 31) => true,
            (1992, Month::December, 31) => true,
            (1993, Month::December, 31) => true,
            (1994, Month::December, 30) => true,
            (1995, Month::December, 29) => true,
            (1996, Month::December, 31) => true,
            (1997, Month::December, 31) => true,
            (1998, Month::December, 31) => true,
            (1999, Month::December, 31) => true,
            (2000, Month::December, 29) => true,
            (2001, Month::December, 31) => true,
            (2002, Month::December, 31) => true,
            (2003, Month::December, 31) => true,
            (2004, Month::December, 31) => true,
            (2005, Month::December, 30) => true,
            (2006, Month::December, 29) => true,
            (2007, Month::December, 31) => true,
            (2008, Month::December, 31) => true,
            (2009, Month::December, 31) => true,
            (2010, Month::December, 31) => true,
            (2011, Month::December, 30) => true,
            (2012, Month::December, 31) => true,
            (2013, Month::December, 31) => true,
            (2014, Month::December, 31) => true,
            (2015, Month::December, 31) => true,
            (2016, Month::December, 30) => true,
            (2017, Month::December, 29) => true,
            (2018, Month::December, 31) => true,
            (2019, Month::December, 31) => true,
            (2020, Month::December, 31) => true,
            (2021, Month::December, 31) => true,
            (2022, Month::December, 30) => true,
            (2023, Month::December, 29) => true,
            (2024, Month::December, 31) => true,
            (2025, Month::December, 31) => true,
            (2026, Month::December, 31) => true,
            (2027, Month::December, 31) => true,
            (2028, Month::December, 29) => true,
            (2029, Month::December, 31) => true,
            (2030, Month::December, 31) => true,
            (2031, Month::December, 31) => true,
            (2032, Month::December, 31) => true,
            (2033, Month::December, 30) => true,
            (2034, Month::December, 29) => true,
            (2035, Month::December, 31) => true,
            (2036, Month::December, 31) => true,
            (2037, Month::December, 31) => true,
            (2038, Month::December, 31) => true,
            (2039, Month::December, 30) => true,
            (2040, Month::December, 31) => true,
            (2041, Month::December, 31) => true,
            (2042, Month::December, 31) => true,
            (2043, Month::December, 31) => true,
            (2044, Month::December, 30) => true,
            (2045, Month::December, 29) => true,
            (2046, Month::December, 31) => true,
            (2047, Month::December, 31) => true,
            (2048, Month::December, 31) => true,
            (2049, Month::December, 31) => true,
            (2050, Month::December, 30) => true,
            (2051, Month::December, 29) => true,
            (2052, Month::December, 31) => true,
            (2053, Month::December, 31) => true,
            (2054, Month::December, 31) => true,
            (2055, Month::December, 31) => true,
            (2056, Month::December, 29) => true,
            (2057, Month::December, 31) => true,
            (2058, Month::December, 31) => true,
            (2059, Month::December, 31) => true,
            (2060, Month::December, 31) => true,
            (2061, Month::December, 30) => true,
            (2062, Month::December, 29) => true,
            (2063, Month::December, 31) => true,
            (2064, Month::December, 31) => true,
            (2065, Month::December, 31) => true,
            (2066, Month::December, 31) => true,
            (2067, Month::December, 30) => true,
            (2068, Month::December, 31) => true,
            (2069, Month::December, 31) => true,
            (2070, Month::December, 31) => true,
            (2071, Month::December, 31) => true,
            (2072, Month::December, 30) => true,
            (2073, Month::December, 29) => true,
            (2074, Month::December, 31) => true,
            (2075, Month::December, 31) => true,
            (2076, Month::December, 31) => true,
            (2077, Month::December, 31) => true,
            (2078, Month::December, 30) => true,
            (2079, Month::December, 29) => true,
            (2080, Month::December, 31) => true,
            (2081, Month::December, 31) => true,
            (2082, Month::December, 31) => true,
            (2083, Month::December, 31) => true,
            (2084, Month::December, 29) => true,
            (2085, Month::December, 31) => true,
            (2086, Month::December, 31) => true,
            (2087, Month::December, 31) => true,
            (2088, Month::December, 31) => true,
            (2089, Month::December, 30) => true,
            (2090, Month::December, 29) => true,
            (2091, Month::December, 31) => true,
            (2092, Month::December, 31) => true,
            (2093, Month::December, 31) => true,
            (2094, Month::December, 31) => true,
            (2095, Month::December, 30) => true,
            (2096, Month::December, 31) => true,
            (2097, Month::December, 31) => true,
            (2098, Month::December, 31) => true,
            (2099, Month::December, 31) => true,
            (2100, Month::December, 31) => true,
            _ => false,
        };
        res
    }

    fn is_temporary_holiday(&self, date: &OffsetDateTime) -> bool {
        let year = date.year();
        let month = date.month();
        let day = date.day();

        let res = match (year, month, day) {
            // 2023
            (2023, Month::January, 24) => true,
            (2023, Month::May, 1) => true,
            (2023, Month::May, 29) => true,
            (2023, Month::October, 2) => true,
            // 2024
            (2024, Month::February, 12) => true,
            (2024, Month::May, 6) => true,
            _ => false,
        };
        res
    }
    fn is_holiday(&self, date: &OffsetDateTime) -> bool {
        if self.is_lunar_new_year(&date) // 'lunar' new year's day
        || self.is_buddha_birthday(&date) // buddha's birthday
        || self.is_chuseok(&date) // chuseok
        || self.is_labour_day(&date) // labour day
        || (date.month() == Month::January && date.day() == 1) // solar new year
        || (date.month() == Month::March && date.day() == 1) // independence movement day
        || (date.month() == Month::May && date.day() == 5) // children's day
        || (date.month() == Month::June && date.day() == 6) // memorial day
        || (date.month() == Month::August && date.day() == 15) // liberation day
        || (date.month() == Month::October && date.day() == 3) // national foundation day
        || (date.month() == Month::October && date.day() == 9) // hangul day
        || (date.month() == Month::December && date.day() == 25) // christmas
        {
            return true;
        }

        if self.is_temporary_holiday(&date)
        {
            return true;
        }

        match self {
            SouthKoreaType::Krx => {
                if self.is_last_business_day_of_year(&date) {
                    return true;
                }
                else {
                    return false;
                }
            },
            SouthKoreaType::Settlement => {return false},
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SouthKorea {
    name: String,
    utc_offset: UtcOffset,
    specific_type: SouthKoreaType,
    holiday_adder: Vec<Date>,
    holiday_remover: Vec<Date>,
}

impl SouthKorea {
    pub fn new(specific_type: SouthKoreaType) -> Self {
        let type_name = match specific_type{
                                SouthKoreaType::Krx => "KRX",
                                SouthKoreaType::Settlement => "Settlement",
        };
        let name = format!("South Korea ({})", type_name);
        Self {
            name: name,
            utc_offset: UtcOffset::from_hms(9, 0, 0).expect("valid offset"),
            specific_type: specific_type,
            holiday_adder: Vec::new(),
            holiday_remover: Vec::new(),
        }
    }
}

impl CalendarTrait for SouthKorea {
    fn calendar_name(&self) -> &String {
        &self.name
    }   
    
    fn add_holidays(&mut self, date: &Date) -> Result<()> {
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

        self._display_holidays(&start_date, &end_date, include_weekend);
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;
    use crate::time::conventions::BusinessDayConvention;

    //name test
    #[test]
    fn test_south_korea_name() {
        let calendar = SouthKorea::new(SouthKoreaType::Krx);
        assert_eq!(calendar.calendar_name(), "South Korea (KRX)");
        let calendar = SouthKorea::new(SouthKoreaType::Settlement);
        assert_eq!(calendar.calendar_name(), "South Korea (Settlement)");
    }

    #[test]
    fn test_south_korea_calendar() {
        let mut calendar = SouthKorea::new(SouthKoreaType::Krx);
        
        let dt = datetime!(2024-2-12 0:0:0 +09:00);
        assert!(calendar.is_holiday(&dt));
        let dt = datetime!(2024-1-29 0:0:0 +09:00);
        assert!(!calendar.is_holiday(&dt));

        let dt = datetime!(2024-5-5 0:0:0 +09:00);
        assert!(calendar.is_holiday(&dt));
        let dt = datetime!(2024-5-7 0:0:0 +09:00);

        assert!(!calendar.is_holiday(&dt));
 
        assert!(calendar.is_holiday(&datetime!(2024-05-01 0:0:0 +09:00)));
        assert!(!calendar.is_holiday(&datetime!(2024-05-02 0:0:0 +09:00)));

        assert!(calendar.is_holiday(&datetime!(2024-09-28 0:0:0 +09:00)));
        assert!(!calendar.is_holiday(&datetime!(2024-09-30 0:0:0 +09:00)));
        
        let test_date = datetime!(2024-01-29 0:0:0 +09:00);
        assert!(!calendar.is_holiday(&test_date));
        calendar.add_holidays(&test_date.date());
        assert!(calendar.is_holiday(&test_date));
        calendar.remove_holidays(&test_date.date());
        assert!(!calendar.is_holiday(&test_date));
    }

    //unittest for adjusted business day
    
    #[test]
    fn test_south_korea_calendar_adjusted_business_day() {
        let calendar = SouthKorea::new(SouthKoreaType::Krx);
        
        let test_date = datetime!(2023-12-29 0:0:0 +09:00);
        assert_eq!(calendar.adjust(&test_date, &BusinessDayConvention::Unadjusted), datetime!(2023-12-29 0:0:0 +09:00));

        let test_date = datetime!(2023-12-29 0:0:0 +09:00);
        assert_eq!(calendar.adjust(&test_date, &BusinessDayConvention::Following), datetime!(2024-01-02 0:0:0 +09:00));
        assert_eq!(calendar.adjust(&test_date, &BusinessDayConvention::ModifiedFollowing), datetime!(2023-12-28 0:0:0 +09:00));

        let test_date = datetime!(2024-01-01 0:0:0 +09:00);
        assert_eq!(calendar.adjust(&test_date, &BusinessDayConvention::Preceding), datetime!(2023-12-28 0:0:0 +09:00));
        assert_eq!(calendar.adjust(&test_date, &BusinessDayConvention::ModifiedPreceding), datetime!(2024-01-02 0:0:0 +09:00));

        let calendar = SouthKorea::new(SouthKoreaType::Settlement);

        let test_date = datetime!(2024-01-01 0:0:0 +09:00);
        assert_eq!(calendar.adjust(&test_date, &BusinessDayConvention::Preceding), datetime!(2023-12-29 0:0:0 +09:00));
    }
}


