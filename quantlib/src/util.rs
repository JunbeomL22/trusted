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
    *parts.last().unwrap_or(&full_name)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
