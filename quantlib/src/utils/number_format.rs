use crate::definitions::Real;
use std::fmt;

pub fn write_number_with_commas(f: &mut fmt::Formatter<'_>, number: Real) -> fmt::Result {
    let number_str = format!("{:.1}", number);
    let parts: Vec<&str> = number_str.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = if parts.len() > 1 { parts[1] } else { "" };
    
    let mut comma_separated = String::new();
    
    let is_negative = integer_part.starts_with('-');
    let abs_integer_part = if is_negative { &integer_part[1..] } else { integer_part };
    
    for (count, c) in abs_integer_part.chars().rev().enumerate() {
        if count > 0 && count % 3 == 0 {
            comma_separated.insert(0, ',');
        }
        comma_separated.insert(0, c);
    }
    
    if is_negative {
        comma_separated.insert(0, '-');
    }
    
    if !decimal_part.is_empty() {
        comma_separated.push('.');
        comma_separated.push_str(decimal_part);
    }
    
    write!(f, "{}", comma_separated)
}


pub fn formatted_number(number: Real) -> String {
    let number_str = format!("{:.0}", number);
    let parts: Vec<&str> = number_str.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = if parts.len() > 1 { parts[1] } else { "" };
    
    let mut comma_separated = String::new();
    
    let is_negative = integer_part.starts_with('-');
    let abs_integer_part = if is_negative { &integer_part[1..] } else { integer_part };
    
    for (count, c) in abs_integer_part.chars().rev().enumerate() {
        if count > 0 && count % 3 == 0 {
            comma_separated.insert(0, ',');
        }
        comma_separated.insert(0, c);
    }
    
    if is_negative {
        comma_separated.insert(0, '-');
    }
    
    if !decimal_part.is_empty() {
        comma_separated.push('.');
        comma_separated.push_str(decimal_part);
    }
    
    comma_separated
}