use crate::types::precision::Precision;
use anyhow::{Result, anyhow};

pub fn get_precision_handler(
    precision: u8, 
    data_length: u8,
    allow_negative: bool,
    unused_part_length: u8,
) -> Result<Precision> {
    if precision >= data_length {
        let err = || anyhow!("precision: {precision} >= data_length: {data_length}", precision = precision, data_length = data_length);
        return Err(err());
    }

    let mut integer_part_length = data_length - precision;
    if precision {
        integer_part_length -= 1; // remove the . separator
    }

    if allow_negative {
        integer_part_length -= 1; // remove the - sign
    }

    integer_part_length -= unused_part_length;
    
    let precision_system_number: u8 = if integer_part_length > 18 { 0 
    } else if integer_part_length > 15 { 3
    } else if integer_part_length > 12 { 6
    } else if integer_part_length > 9 { 9
    } else {
        let err = || anyhow!("integer_part_length: {} is not supported", integer_part_length);
        return Err(err());
    };

    let res: Precision = match (precision, precision_system_number) {
        (0, 3) => Precision::Prec0_3,
        (2, 3) => Precision::Prec2_3,
        (3, 3) => Precision::Prec3_3,
        (6, 6) => Precision::Prec6_6,
        (9, 9) => Precision::Prec9_9,
        _ => {
            let err = || anyhow!("precision: {precision} and precision_system_number: {precision_system_number} is not supported", precision = precision, precision_system_number = precision_system_number);
            return Err(err());
        }
    };

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_precision() -> Result<()> {
        assert_eq!(get_precision(0, 3, false, 0)?, Precision::Prec0_3);
        assert_eq!(get_precision(2, 3, false, 0)?, Precision::Prec2_3);
        assert_eq!(get_precision(3, 3, false, 0)?, Precision::Prec3_3);
        assert_eq!(get_precision(6, 6, false, 0)?, Precision::Prec6_6);
        assert_eq!(get_precision(9, 9, false, 0)?, Precision::Prec9_9);
}