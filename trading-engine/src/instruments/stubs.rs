use crate::types::precision::PrecisionHelper;
use anyhow::{Result, Error, anyhow};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrecisionCreationError {
    #[error(
        "get_precision_helper => invalid precision: \n\
        digit_length: {:?}\n\
        decimal_point_length: {:?}\n\
        include_negative: {:?}\n\
        all_size_for_checker: {:?}",
        .0, .1, .2, .3
    )]
    InvalidPrecision(u8, u8, bool, u8),
    #[error(
        "get_precision_helper => too big digit-length: \n\
        digit_length: {:?}\n\
        decimal_point_length: {:?}\n\
        include_negative: {:?}\n\
        all_size_for_checker: {:?}",
        .0, .1, .2, .3
    )]
    DigitLength(u8, u8, bool, u8),
}



pub fn get_precision(
    digit_length: u8,
    decimal_point_length: u8,
    include_negative: bool,
    all_size_for_checker: u8,
) -> Result<PrecisionHelper> {
    let mut check_size: u8 = if decimal_point_length == 0 {
        digit_length
    } else {
        digit_length + decimal_point_length + 1
    };

    check_size += if include_negative { 1 } else { 0 };

    if check_size != all_size_for_checker {
        let error = || anyhow!(
            "({}:{}) size check failed: {} > {}",
            file!(), line!(), check_size, all_size_for_checker
        );
        Err(error().into())
    } else if decimal_point_length == 0 {
        if (0..=15).contains(&digit_length) {
            Ok(PrecisionHelper::Prec0_3)
        } else if (16..=18).contains(&digit_length) {
            Ok(PrecisionHelper::Prec0_0)
        } else {
            let error = || PrecisionCreationError::DigitLength(
                digit_length, decimal_point_length, include_negative, all_size_for_checker
            );
            Err(error().into())
        }
    } else if decimal_point_length == 2 {
        if (0..=15).contains(&digit_length) {
            Ok(PrecisionHelper::Prec2_3)
        } else {
            let error = || PrecisionCreationError::DigitLength(
                digit_length, decimal_point_length, include_negative, all_size_for_checker
            );
            Err(error().into())
        }
    } else if decimal_point_length == 3 {
        if (0..=15).contains(&digit_length) {
            Ok(PrecisionHelper::Prec3_3)
        } else {
            let error = || PrecisionCreationError::DigitLength(
                digit_length, decimal_point_length, include_negative, all_size_for_checker
            );
            Err(error().into())
        }
    } else {
        let error = || PrecisionCreationError::InvalidPrecision(
            digit_length, decimal_point_length, include_negative, all_size_for_checker
        );
        Err(error().into())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_precision() -> Result<()> {
        let precision = get_precision(15, 3, false, 20);
        assert!(precision.is_err());

        let precision = get_precision(15, 3, false, 19);
        assert!(precision.is_ok());
        assert_eq!(precision?, PrecisionHelper::Prec3_3);

        Ok(())
    }
}