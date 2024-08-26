use crate::time::calendar_trait::CalendarTrait;
use crate::time::constants::{FIRST_LUNAR_NEWYEAR, KOREAN_LUNAR_NEWYEARS, LAST_LUNAR_NEWYEAR};
use crate::time::holiday::Holidays;
//
use anyhow::Result;
use log::warn;
use serde::{Deserialize, Serialize};
use time::{Date, Duration, Month, OffsetDateTime, UtcOffset};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum SouthKoreaType {
    Krx,
    Settlement,
}

impl SouthKoreaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SouthKoreaType::Krx => "KRX",
            SouthKoreaType::Settlement => "Settlement",
        }
    }
    
    fn is_lunar_new_year(&self, date: &OffsetDateTime) -> bool {
        let date = date.date();
        let year = date.year();

        if (year as usize > LAST_LUNAR_NEWYEAR) || (FIRST_LUNAR_NEWYEAR > year as usize) {
            warn!("Lunar New Year is not available for the year {}", year);
            return false;
        }

        let solar_ordinal = KOREAN_LUNAR_NEWYEARS[year as usize - FIRST_LUNAR_NEWYEAR];

        let lunar_new_year_day = Date::from_ordinal_date(year, solar_ordinal).unwrap();

        if (lunar_new_year_day == date)
            || (lunar_new_year_day + Duration::days(1) == date)
            || (lunar_new_year_day - Duration::days(1) == date)
        {
            return true;
        }
        false
    }

    fn is_labour_day(&self, date: &OffsetDateTime) -> bool {
        let month = date.month();
        let day = date.day();

        if month == Month::May && day == 1 {
            return true;
        }
        false
    }

    fn is_buddha_birthday(&self, date: &OffsetDateTime) -> bool {
        let date = date.date();
        let year = date.year();
        let month = date.month();
        let day = date.day();

        if !(1951..=2050).contains(&year) {
            warn!("Buddha's Birthday is not implemented for the year {}", year);
            return false;
        }

        matches!(
            (year, month, day),
            (1951, Month::May, 13)
                | (1952, Month::May, 1)
                | (1953, Month::May, 20)
                | (1954, Month::May, 10)
                | (1955, Month::May, 29)
                | (1956, Month::May, 17)
                | (1957, Month::May, 7)
                | (1958, Month::May, 26)
                | (1959, Month::May, 15)
                | (1960, Month::May, 3)
                | (1961, Month::May, 22)
                | (1962, Month::May, 11)
                | (1963, Month::May, 1)
                | (1964, Month::May, 19)
                | (1965, Month::May, 8)
                | (1966, Month::May, 27)
                | (1967, Month::May, 16)
                | (1968, Month::May, 5)
                | (1969, Month::May, 23)
                | (1970, Month::May, 12)
                | (1971, Month::May, 2)
                | (1972, Month::May, 20)
                | (1973, Month::May, 10)
                | (1974, Month::April, 29)
                | (1975, Month::May, 18)
                | (1976, Month::May, 6)
                | (1977, Month::May, 25)
                | (1978, Month::May, 14)
                | (1979, Month::May, 3)
                | (1980, Month::May, 21)
                | (1981, Month::May, 11)
                | (1982, Month::May, 1)
                | (1983, Month::May, 20)
                | (1984, Month::May, 8)
                | (1985, Month::May, 27)
                | (1986, Month::May, 16)
                | (1987, Month::May, 5)
                | (1988, Month::May, 23)
                | (1989, Month::May, 12)
                | (1990, Month::May, 2)
                | (1991, Month::May, 21)
                | (1992, Month::May, 10)
                | (1993, Month::May, 28)
                | (1994, Month::May, 18)
                | (1995, Month::May, 7)
                | (1996, Month::May, 24)
                | (1997, Month::May, 14)
                | (1998, Month::May, 3)
                | (1999, Month::May, 22)
                | (2000, Month::May, 11)
                | (2001, Month::May, 1)
                | (2002, Month::May, 19)
                | (2003, Month::May, 8)
                | (2004, Month::May, 26)
                | (2005, Month::May, 15)
                | (2006, Month::May, 5)
                | (2007, Month::May, 24)
                | (2008, Month::May, 12)
                | (2009, Month::May, 2)
                | (2010, Month::May, 21)
                | (2011, Month::May, 10)
                | (2012, Month::May, 28)
                | (2013, Month::May, 17)
                | (2014, Month::May, 6)
                | (2015, Month::May, 25)
                | (2016, Month::May, 14)
                | (2017, Month::May, 3)
                | (2018, Month::May, 22)
                | (2019, Month::May, 12)
                | (2020, Month::April, 30)
                | (2021, Month::May, 19)
                | (2022, Month::May, 8)
                | (2023, Month::May, 27)
                | (2024, Month::May, 15)
                | (2025, Month::May, 5)
                | (2026, Month::May, 24)
                | (2027, Month::May, 13)
                | (2028, Month::May, 2)
                | (2029, Month::May, 20)
                | (2030, Month::May, 9)
                | (2031, Month::May, 28)
                | (2032, Month::May, 16)
                | (2033, Month::May, 6)
                | (2034, Month::May, 25)
                | (2035, Month::May, 15)
                | (2036, Month::May, 3)
                | (2037, Month::May, 22)
                | (2038, Month::May, 11)
                | (2039, Month::April, 30)
                | (2040, Month::May, 18)
                | (2041, Month::May, 7)
                | (2042, Month::May, 26)
                | (2043, Month::May, 16)
                | (2044, Month::May, 5)
                | (2045, Month::May, 24)
                | (2046, Month::May, 13)
                | (2047, Month::May, 2)
                | (2048, Month::May, 20)
                | (2049, Month::May, 9)
                | (2050, Month::May, 28)
        )
    }

    fn is_chuseok(&self, date: &OffsetDateTime) -> bool {
        let year = date.year();
        let month = date.month();
        let day = date.day();

        if !(1951..=2050).contains(&year) {
            warn!("Chuseok is not implemented for the year {}", year);
            return false;
        }

        matches!(
            (year, month, day),
            (1951, Month::September, 14)
                | (1951, Month::September, 15)
                | (1951, Month::September, 16)
                | (1952, Month::October, 2)
                | (1952, Month::October, 3)
                | (1952, Month::October, 4)
                | (1953, Month::September, 21)
                | (1953, Month::September, 22)
                | (1953, Month::September, 23)
                | (1954, Month::September, 10)
                | (1954, Month::September, 11)
                | (1954, Month::September, 12)
                | (1955, Month::September, 29)
                | (1955, Month::September, 30)
                | (1955, Month::October, 1)
                | (1956, Month::September, 18)
                | (1956, Month::September, 19)
                | (1956, Month::September, 20)
                | (1957, Month::September, 7)
                | (1957, Month::September, 8)
                | (1957, Month::September, 9)
                | (1958, Month::September, 26)
                | (1958, Month::September, 27)
                | (1958, Month::September, 28)
                | (1959, Month::September, 16)
                | (1959, Month::September, 17)
                | (1959, Month::September, 18)
                | (1960, Month::October, 4)
                | (1960, Month::October, 5)
                | (1960, Month::October, 6)
                | (1961, Month::September, 23)
                | (1961, Month::September, 24)
                | (1961, Month::September, 25)
                | (1962, Month::September, 12)
                | (1962, Month::September, 13)
                | (1962, Month::September, 14)
                | (1963, Month::October, 1)
                | (1963, Month::October, 2)
                | (1963, Month::October, 3)
                | (1964, Month::September, 19)
                | (1964, Month::September, 20)
                | (1964, Month::September, 21)
                | (1965, Month::September, 9)
                | (1965, Month::September, 10)
                | (1965, Month::September, 11)
                | (1966, Month::September, 28)
                | (1966, Month::September, 29)
                | (1966, Month::September, 30)
                | (1967, Month::September, 17)
                | (1967, Month::September, 18)
                | (1967, Month::September, 19)
                | (1968, Month::October, 5)
                | (1968, Month::October, 6)
                | (1968, Month::October, 7)
                | (1969, Month::September, 25)
                | (1969, Month::September, 26)
                | (1969, Month::September, 27)
                | (1970, Month::September, 14)
                | (1970, Month::September, 15)
                | (1970, Month::September, 16)
                | (1971, Month::October, 2)
                | (1971, Month::October, 3)
                | (1971, Month::October, 4)
                | (1972, Month::September, 21)
                | (1972, Month::September, 22)
                | (1972, Month::September, 23)
                | (1973, Month::September, 10)
                | (1973, Month::September, 11)
                | (1973, Month::September, 12)
                | (1974, Month::September, 29)
                | (1974, Month::September, 30)
                | (1974, Month::October, 1)
                | (1975, Month::September, 19)
                | (1975, Month::September, 20)
                | (1975, Month::September, 21)
                | (1976, Month::September, 7)
                | (1976, Month::September, 8)
                | (1976, Month::September, 9)
                | (1977, Month::September, 26)
                | (1977, Month::September, 27)
                | (1977, Month::September, 28)
                | (1978, Month::September, 16)
                | (1978, Month::September, 17)
                | (1978, Month::September, 18)
                | (1979, Month::October, 4)
                | (1979, Month::October, 5)
                | (1979, Month::October, 6)
                | (1980, Month::September, 22)
                | (1980, Month::September, 23)
                | (1980, Month::September, 24)
                | (1981, Month::September, 11)
                | (1981, Month::September, 12)
                | (1981, Month::September, 13)
                | (1982, Month::September, 30)
                | (1982, Month::October, 1)
                | (1982, Month::October, 2)
                | (1983, Month::September, 20)
                | (1983, Month::September, 21)
                | (1983, Month::September, 22)
                | (1984, Month::September, 9)
                | (1984, Month::September, 10)
                | (1984, Month::September, 11)
                | (1985, Month::September, 28)
                | (1985, Month::September, 29)
                | (1985, Month::September, 30)
                | (1986, Month::September, 17)
                | (1986, Month::September, 18)
                | (1986, Month::September, 19)
                | (1987, Month::October, 6)
                | (1987, Month::October, 7)
                | (1987, Month::October, 8)
                | (1988, Month::September, 24)
                | (1988, Month::September, 25)
                | (1988, Month::September, 26)
                | (1989, Month::September, 13)
                | (1989, Month::September, 14)
                | (1989, Month::September, 15)
                | (1990, Month::October, 2)
                | (1990, Month::October, 3)
                | (1990, Month::October, 4)
                | (1991, Month::September, 21)
                | (1991, Month::September, 22)
                | (1991, Month::September, 23)
                | (1992, Month::September, 10)
                | (1992, Month::September, 11)
                | (1992, Month::September, 12)
                | (1993, Month::September, 29)
                | (1993, Month::September, 30)
                | (1993, Month::October, 1)
                | (1994, Month::September, 19)
                | (1994, Month::September, 20)
                | (1994, Month::September, 21)
                | (1995, Month::September, 8)
                | (1995, Month::September, 9)
                | (1995, Month::September, 10)
                | (1996, Month::September, 26)
                | (1996, Month::September, 27)
                | (1996, Month::September, 28)
                | (1997, Month::September, 15)
                | (1997, Month::September, 16)
                | (1997, Month::September, 17)
                | (1998, Month::October, 4)
                | (1998, Month::October, 5)
                | (1998, Month::October, 6)
                | (1999, Month::September, 23)
                | (1999, Month::September, 24)
                | (1999, Month::September, 25)
                | (2000, Month::September, 11)
                | (2000, Month::September, 12)
                | (2000, Month::September, 13)
                | (2001, Month::September, 30)
                | (2001, Month::October, 1)
                | (2001, Month::October, 2)
                | (2002, Month::September, 20)
                | (2002, Month::September, 21)
                | (2002, Month::September, 22)
                | (2003, Month::September, 10)
                | (2003, Month::September, 11)
                | (2003, Month::September, 12)
                | (2004, Month::September, 27)
                | (2004, Month::September, 28)
                | (2004, Month::September, 29)
                | (2005, Month::September, 17)
                | (2005, Month::September, 18)
                | (2005, Month::September, 19)
                | (2006, Month::October, 5)
                | (2006, Month::October, 6)
                | (2006, Month::October, 7)
                | (2007, Month::September, 24)
                | (2007, Month::September, 25)
                | (2007, Month::September, 26)
                | (2008, Month::September, 13)
                | (2008, Month::September, 14)
                | (2008, Month::September, 15)
                | (2009, Month::October, 2)
                | (2009, Month::October, 3)
                | (2009, Month::October, 4)
                | (2010, Month::September, 21)
                | (2010, Month::September, 22)
                | (2010, Month::September, 23)
                | (2011, Month::September, 11)
                | (2011, Month::September, 12)
                | (2011, Month::September, 13)
                | (2012, Month::September, 29)
                | (2012, Month::September, 30)
                | (2012, Month::October, 1)
                | (2013, Month::September, 18)
                | (2013, Month::September, 19)
                | (2013, Month::September, 20)
                | (2014, Month::September, 7)
                | (2014, Month::September, 8)
                | (2014, Month::September, 9)
                | (2015, Month::September, 26)
                | (2015, Month::September, 27)
                | (2015, Month::September, 28)
                | (2016, Month::September, 14)
                | (2016, Month::September, 15)
                | (2016, Month::September, 16)
                | (2017, Month::October, 3)
                | (2017, Month::October, 4)
                | (2017, Month::October, 5)
                | (2018, Month::September, 23)
                | (2018, Month::September, 24)
                | (2018, Month::September, 25)
                | (2019, Month::September, 12)
                | (2019, Month::September, 13)
                | (2019, Month::September, 14)
                | (2020, Month::September, 30)
                | (2020, Month::October, 1)
                | (2020, Month::October, 2)
                | (2021, Month::September, 20)
                | (2021, Month::September, 21)
                | (2021, Month::September, 22)
                | (2022, Month::September, 9)
                | (2022, Month::September, 10)
                | (2022, Month::September, 11)
                | (2023, Month::September, 28)
                | (2023, Month::September, 29)
                | (2023, Month::September, 30)
                | (2024, Month::September, 16)
                | (2024, Month::September, 17)
                | (2024, Month::September, 18)
                | (2025, Month::October, 5)
                | (2025, Month::October, 6)
                | (2025, Month::October, 7)
                | (2026, Month::September, 24)
                | (2026, Month::September, 25)
                | (2026, Month::September, 26)
                | (2027, Month::September, 14)
                | (2027, Month::September, 15)
                | (2027, Month::September, 16)
                | (2028, Month::October, 2)
                | (2028, Month::October, 3)
                | (2028, Month::October, 4)
                | (2029, Month::September, 21)
                | (2029, Month::September, 22)
                | (2029, Month::September, 23)
                | (2030, Month::September, 11)
                | (2030, Month::September, 12)
                | (2030, Month::September, 13)
                | (2031, Month::September, 30)
                | (2031, Month::October, 1)
                | (2031, Month::October, 2)
                | (2032, Month::September, 18)
                | (2032, Month::September, 19)
                | (2032, Month::September, 20)
                | (2033, Month::September, 7)
                | (2033, Month::September, 8)
                | (2033, Month::September, 9)
                | (2034, Month::September, 26)
                | (2034, Month::September, 27)
                | (2034, Month::September, 28)
                | (2035, Month::September, 15)
                | (2035, Month::September, 16)
                | (2035, Month::September, 17)
                | (2036, Month::October, 3)
                | (2036, Month::October, 4)
                | (2036, Month::October, 5)
                | (2037, Month::September, 23)
                | (2037, Month::September, 24)
                | (2037, Month::September, 25)
                | (2038, Month::September, 12)
                | (2038, Month::September, 13)
                | (2038, Month::September, 14)
                | (2039, Month::October, 1)
                | (2039, Month::October, 2)
                | (2039, Month::October, 3)
                | (2040, Month::September, 20)
                | (2040, Month::September, 21)
                | (2040, Month::September, 22)
                | (2041, Month::September, 9)
                | (2041, Month::September, 10)
                | (2041, Month::September, 11)
                | (2042, Month::September, 27)
                | (2042, Month::September, 28)
                | (2042, Month::September, 29)
                | (2043, Month::September, 16)
                | (2043, Month::September, 17)
                | (2043, Month::September, 18)
                | (2044, Month::October, 4)
                | (2044, Month::October, 5)
                | (2044, Month::October, 6)
                | (2045, Month::September, 24)
                | (2045, Month::September, 25)
                | (2045, Month::September, 26)
                | (2046, Month::September, 14)
                | (2046, Month::September, 15)
                | (2046, Month::September, 16)
                | (2047, Month::October, 3)
                | (2047, Month::October, 4)
                | (2047, Month::October, 5)
                | (2048, Month::September, 21)
                | (2048, Month::September, 22)
                | (2048, Month::September, 23)
                | (2049, Month::September, 10)
                | (2049, Month::September, 11)
                | (2049, Month::September, 12)
                | (2050, Month::September, 29)
                | (2050, Month::September, 30)
                | (2050, Month::October, 1)
        )
    }
}

impl Holidays for SouthKoreaType {
    fn is_last_business_day_of_year(&self, date: &OffsetDateTime) -> bool {
        // hard coded for avoiding infinite loop
        let year = date.year();
        let month = date.month();
        let day = date.day();

        if !(1951..=2050).contains(&year) {
            warn!(
                "Last business day of the year is not implemented for the year {}",
                year
            );
            return false;
        }

        matches!(
            (year, month, day),
            (1951, Month::December, 31)
                | (1952, Month::December, 31)
                | (1953, Month::December, 31)
                | (1954, Month::December, 31)
                | (1955, Month::December, 30)
                | (1956, Month::December, 31)
                | (1957, Month::December, 31)
                | (1958, Month::December, 31)
                | (1959, Month::December, 31)
                | (1960, Month::December, 30)
                | (1961, Month::December, 29)
                | (1962, Month::December, 31)
                | (1963, Month::December, 31)
                | (1964, Month::December, 31)
                | (1965, Month::December, 31)
                | (1966, Month::December, 30)
                | (1967, Month::December, 29)
                | (1968, Month::December, 31)
                | (1969, Month::December, 31)
                | (1970, Month::December, 31)
                | (1971, Month::December, 31)
                | (1972, Month::December, 29)
                | (1973, Month::December, 31)
                | (1974, Month::December, 31)
                | (1975, Month::December, 31)
                | (1976, Month::December, 31)
                | (1977, Month::December, 30)
                | (1978, Month::December, 29)
                | (1979, Month::December, 31)
                | (1980, Month::December, 31)
                | (1981, Month::December, 31)
                | (1982, Month::December, 31)
                | (1983, Month::December, 30)
                | (1984, Month::December, 31)
                | (1985, Month::December, 31)
                | (1986, Month::December, 31)
                | (1987, Month::December, 31)
                | (1988, Month::December, 30)
                | (1989, Month::December, 29)
                | (1990, Month::December, 31)
                | (1991, Month::December, 31)
                | (1992, Month::December, 31)
                | (1993, Month::December, 31)
                | (1994, Month::December, 30)
                | (1995, Month::December, 29)
                | (1996, Month::December, 31)
                | (1997, Month::December, 31)
                | (1998, Month::December, 31)
                | (1999, Month::December, 31)
                | (2000, Month::December, 29)
                | (2001, Month::December, 31)
                | (2002, Month::December, 31)
                | (2003, Month::December, 31)
                | (2004, Month::December, 31)
                | (2005, Month::December, 30)
                | (2006, Month::December, 29)
                | (2007, Month::December, 31)
                | (2008, Month::December, 31)
                | (2009, Month::December, 31)
                | (2010, Month::December, 31)
                | (2011, Month::December, 30)
                | (2012, Month::December, 31)
                | (2013, Month::December, 31)
                | (2014, Month::December, 31)
                | (2015, Month::December, 31)
                | (2016, Month::December, 30)
                | (2017, Month::December, 29)
                | (2018, Month::December, 31)
                | (2019, Month::December, 31)
                | (2020, Month::December, 31)
                | (2021, Month::December, 31)
                | (2022, Month::December, 30)
                | (2023, Month::December, 29)
                | (2024, Month::December, 31)
                | (2025, Month::December, 31)
                | (2026, Month::December, 31)
                | (2027, Month::December, 31)
                | (2028, Month::December, 29)
                | (2029, Month::December, 31)
                | (2030, Month::December, 31)
                | (2031, Month::December, 31)
                | (2032, Month::December, 31)
                | (2033, Month::December, 30)
                | (2034, Month::December, 29)
                | (2035, Month::December, 31)
                | (2036, Month::December, 31)
                | (2037, Month::December, 31)
                | (2038, Month::December, 31)
                | (2039, Month::December, 30)
                | (2040, Month::December, 31)
                | (2041, Month::December, 31)
                | (2042, Month::December, 31)
                | (2043, Month::December, 31)
                | (2044, Month::December, 30)
                | (2045, Month::December, 29)
                | (2046, Month::December, 31)
                | (2047, Month::December, 31)
                | (2048, Month::December, 31)
                | (2049, Month::December, 31)
                | (2050, Month::December, 30)
                | (2051, Month::December, 29)
                | (2052, Month::December, 31)
                | (2053, Month::December, 31)
                | (2054, Month::December, 31)
                | (2055, Month::December, 31)
                | (2056, Month::December, 29)
                | (2057, Month::December, 31)
                | (2058, Month::December, 31)
                | (2059, Month::December, 31)
                | (2060, Month::December, 31)
                | (2061, Month::December, 30)
                | (2062, Month::December, 29)
                | (2063, Month::December, 31)
                | (2064, Month::December, 31)
                | (2065, Month::December, 31)
                | (2066, Month::December, 31)
                | (2067, Month::December, 30)
                | (2068, Month::December, 31)
                | (2069, Month::December, 31)
                | (2070, Month::December, 31)
                | (2071, Month::December, 31)
                | (2072, Month::December, 30)
                | (2073, Month::December, 29)
                | (2074, Month::December, 31)
                | (2075, Month::December, 31)
                | (2076, Month::December, 31)
                | (2077, Month::December, 31)
                | (2078, Month::December, 30)
                | (2079, Month::December, 29)
                | (2080, Month::December, 31)
                | (2081, Month::December, 31)
                | (2082, Month::December, 31)
                | (2083, Month::December, 31)
                | (2084, Month::December, 29)
                | (2085, Month::December, 31)
                | (2086, Month::December, 31)
                | (2087, Month::December, 31)
                | (2088, Month::December, 31)
                | (2089, Month::December, 30)
                | (2090, Month::December, 29)
                | (2091, Month::December, 31)
                | (2092, Month::December, 31)
                | (2093, Month::December, 31)
                | (2094, Month::December, 31)
                | (2095, Month::December, 30)
                | (2096, Month::December, 31)
                | (2097, Month::December, 31)
                | (2098, Month::December, 31)
                | (2099, Month::December, 31)
                | (2100, Month::December, 31)
        )
    }

    fn is_temporary_holiday(&self, date: &OffsetDateTime) -> bool {
        let year = date.year();
        let month = date.month();
        let day = date.day();

        match (year, month, day) {
            // 2023
            (2023, Month::January, 24) => true,
            (2023, Month::May, 1) => true,
            (2023, Month::May, 29) => true,
            (2023, Month::October, 2) => true,
            // 2024
            (2024, Month::February, 12) => true,
            (2024, Month::May, 6) => true,
            _ => false,
        }
    }
    fn is_holiday(&self, date: &OffsetDateTime) -> bool {
        if self.is_lunar_new_year(date) // 'lunar' new year's day
        || self.is_buddha_birthday(date) // buddha's birthday
        || self.is_chuseok(date) // chuseok
        || self.is_labour_day(date) // labour day
        || (date.month() == Month::January && date.day() == 1) // solar new year
        || (date.month() == Month::March && date.day() == 1) // independence movement day
        || (date.month() == Month::May && date.day() == 5) // children's day
        || (date.month() == Month::June && date.day() == 6) // memorial day
        || (date.month() == Month::August && date.day() == 15) // liberation day
        || (date.month() == Month::October && date.day() == 3) // national foundation day
        || (date.month() == Month::October && date.day() == 9) // hangul day
        || (date.month() == Month::December && date.day() == 25)
        // christmas
        {
            return true;
        }

        if self.is_temporary_holiday(date) {
            return true;
        }

        match self {
            SouthKoreaType::Krx => self.is_last_business_day_of_year(date),
            SouthKoreaType::Settlement => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SouthKorea {
    name: String,
    utc_offset: UtcOffset,
    specific_type: SouthKoreaType,
    holiday_adder: Vec<Date>,
    holiday_remover: Vec<Date>,
}

impl SouthKorea {
    pub fn new(specific_type: SouthKoreaType) -> Self {
        let type_name = match specific_type {
            SouthKoreaType::Krx => "KRX",
            SouthKoreaType::Settlement => "Settlement",
        };
        let name = format!("South Korea ({})", type_name);
        Self {
            name,
            utc_offset: UtcOffset::from_hms(9, 0, 0).expect("valid offset"),
            specific_type,
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
        self.specific_type.is_holiday(date)
    }

    fn is_holiday(&self, date: &OffsetDateTime) -> bool {
        let date = date.to_offset(self.utc_offset);
        self._is_holiday(&date)
    }

    fn is_weekend(&self, date: &OffsetDateTime) -> bool {
        let date = date.to_offset(self.utc_offset);
        self._is_weekend(&date)
    }

    fn display_holidays(
        &self,
        start_date: &OffsetDateTime,
        end_date: &OffsetDateTime,
        include_weekend: bool,
    ) {
        let start_date = start_date.to_offset(self.utc_offset);
        let end_date = end_date.to_offset(self.utc_offset);

        self._display_holidays(&start_date, &end_date, include_weekend);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::conventions::BusinessDayConvention;
    use anyhow::Result;
    use time::macros::datetime;

    #[test]
    fn test_south_korea_name() {
        let calendar = SouthKorea::new(SouthKoreaType::Krx);
        assert_eq!(calendar.calendar_name(), "South Korea (KRX)");
        let calendar = SouthKorea::new(SouthKoreaType::Settlement);
        assert_eq!(calendar.calendar_name(), "South Korea (Settlement)");
    }

    #[test]
    fn test_south_korea_calendar() -> Result<()> {
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
        calendar.add_holidays(&test_date.date())?;
        assert!(calendar.is_holiday(&test_date));
        calendar.remove_holidays(&test_date.date())?;
        assert!(!calendar.is_holiday(&test_date));
        Ok(())
    }

    //unittest for adjusted business day

    #[test]
    fn test_south_korea_calendar_adjusted_business_day() -> Result<()> {
        let calendar = SouthKorea::new(SouthKoreaType::Krx);

        let test_date = datetime!(2023-12-29 0:0:0 +09:00);
        assert_eq!(
            calendar.adjust(&test_date, &BusinessDayConvention::Unadjusted)?,
            datetime!(2023-12-29 0:0:0 +09:00)
        );

        let test_date = datetime!(2023-12-29 0:0:0 +09:00);
        assert_eq!(
            calendar.adjust(&test_date, &BusinessDayConvention::Following)?,
            datetime!(2024-01-02 0:0:0 +09:00)
        );
        assert_eq!(
            calendar.adjust(&test_date, &BusinessDayConvention::ModifiedFollowing)?,
            datetime!(2023-12-28 0:0:0 +09:00)
        );

        let test_date = datetime!(2024-01-01 0:0:0 +09:00);
        assert_eq!(
            calendar.adjust(&test_date, &BusinessDayConvention::Preceding)?,
            datetime!(2023-12-28 0:0:0 +09:00)
        );
        assert_eq!(
            calendar.adjust(&test_date, &BusinessDayConvention::ModifiedPreceding)?,
            datetime!(2024-01-02 0:0:0 +09:00)
        );

        let calendar = SouthKorea::new(SouthKoreaType::Settlement);

        let test_date = datetime!(2024-01-01 0:0:0 +09:00);
        assert_eq!(
            calendar.adjust(&test_date, &BusinessDayConvention::Preceding)?,
            datetime!(2023-12-29 0:0:0 +09:00)
        );
        Ok(())
    }
}
