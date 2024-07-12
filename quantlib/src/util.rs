use crate::definitions::Integer;
use ndarray::Array1;
use time::OffsetDateTime;

pub fn min_offsetdatetime(d1: &OffsetDateTime, d2: &OffsetDateTime) -> OffsetDateTime {
    if d1 < d2 {
        *d1
    } else {
        *d2
    }
}

//use crate::definitions::Real;
/// This return the type name of a variable (only name, not the full path)
///
/// # Examples
/// ```
/// use quantlib::util::type_name;
/// let x: i32 = 5;
/// assert_eq!(type_name(&x), "i32");
/// let s: String = "hello".to_string();
/// assert_eq!(type_name(&s), "String");
/// ```
///
pub fn type_name<T>(_: &T) -> &'static str {
    let full_name = std::any::type_name::<T>();
    let parts: Vec<&str> = full_name.split("::").collect();
    parts.last().unwrap_or(&full_name)
}

pub fn is_ndarray_sorted<T>(arr: &Array1<T>) -> bool
where
    T: PartialOrd,
{
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}

pub fn to_yyyymmdd_int(dt: &OffsetDateTime) -> Integer {
    let year = dt.year() as Integer;
    let month = dt.month() as Integer;
    let day = dt.day() as Integer;

    year * 10000 + month * 100 + day
}

pub fn format_duration(secs: f64) -> String {
    let minutes = (secs / 60.0).floor();
    let seconds = secs % 60.0;

    if minutes > 0.0 {
        format!("{:.0}m {:.2}s", minutes, seconds)
    } else {
        format!("{:.2}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use time::macros::datetime;

    #[test]
    fn test_type_name() {
        let x: i32 = 5;
        assert_eq!(type_name(&x), "i32");
        let y: f64 = 5.0;
        assert_eq!(type_name(&y), "f64");
        let z: String = "hello".to_string();
        assert_eq!(type_name(&z), "String");

        enum MockEnum {
            A,
        }
        let a = MockEnum::A;
        assert_eq!(type_name(&a), "MockEnum");

        struct MockStruct {} // Empty struct;
        let s = MockStruct {};
        assert_eq!(type_name(&s), "MockStruct");
    }

    #[test]
    fn test_is_ndarray_sorted() {
        let arr = array![1, 2, 3, 4, 5];
        assert_eq!(is_ndarray_sorted(&arr), true);
        let arr = array![1, 2, 3, 4, 3];
        assert_eq!(is_ndarray_sorted(&arr), false);
    }

    #[test]
    fn test_to_yyyymmdd_int() {
        let dt1 = datetime!(2021-01-01 00:00:00 UTC);
        assert_eq!(to_yyyymmdd_int(&dt1), 20210101);

        let dt2 = datetime!(2022-12-31 23:59:59 UTC);
        assert_eq!(to_yyyymmdd_int(&dt2), 20221231);

        let dt3 = datetime!(2000-02-29 12:34:56 UTC);
        assert_eq!(to_yyyymmdd_int(&dt3), 20000229);

        let dt4 = datetime!(1999-12-31 00:00:00 UTC);
        assert_eq!(to_yyyymmdd_int(&dt4), 19991231);

        let dt5 = datetime!(2100-01-01 00:00:00 UTC);
        assert_eq!(to_yyyymmdd_int(&dt5), 21000101);

        let dt6 = datetime!(1980-06-15 00:00:00 UTC);
        assert_eq!(to_yyyymmdd_int(&dt6), 19800615);

        let dt7 = datetime!(1970-01-01 00:00:00 UTC);
        assert_eq!(to_yyyymmdd_int(&dt7), 19700101);

        let dt8 = datetime!(2020-02-29 00:00:00 UTC);
        assert_eq!(to_yyyymmdd_int(&dt8), 20200229);

        let dt9 = datetime!(2010-10-10 00:00:00 UTC);
        assert_eq!(to_yyyymmdd_int(&dt9), 20101010);

        let dt10 = datetime!(2005-05-05 00:00:00 UTC);
        assert_eq!(to_yyyymmdd_int(&dt10), 20050505);
    }
}
