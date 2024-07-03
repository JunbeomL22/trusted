use crate::types::precision::PrecisionHelper;
use crate::types::base::NumReprCfg;
use anyhow::{Result, Error, anyhow};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrecisionCreationError {
    #[error(
        "get_precision_helper => size check failed: \n\
        digit_length: {:?}\n\
        decimal_point_length: {:?}\n\
        include_negative: {:?}\n\
        total_length: {:?}",
        .0, .1, .2, .3
    
    )]
    SizeError(usize, usize, bool, usize),
    #[error(
        "get_precision_helper => invalid precision: \n\
        digit_length: {:?}\n\
        decimal_point_length: {:?}\n\
        include_negative: {:?}\n\
        total_length: {:?}",
        .0, .1, .2, .3
    )]
    InvalidPrecision(usize, usize, bool, usize),
    #[error(
        "get_precision_helper => too big digit-length: \n\
        digit_length: {:?}\n\
        decimal_point_length: {:?}\n\
        include_negative: {:?}\n\
        total_length: {:?}",
        .0, .1, .2, .3
    )]
    DigitLength(usize, usize, bool, usize),
}

pub fn get_precision_helper(cfg: NumReprCfg) -> Result<PrecisionHelper> {
    let NumReprCfg {
        digit_length,
        decimal_point_length,
        include_negative,
        total_length,
    } = cfg;

    if decimal_point_length == 0 {
        if (0..=15).contains(&digit_length) {
            Ok(PrecisionHelper::Prec0_3)
        } else if (16..=18).contains(&digit_length) {
            Ok(PrecisionHelper::Prec0_0)
        } else {
            let error = || PrecisionCreationError::DigitLength(
                digit_length, decimal_point_length, include_negative, total_length
            );
            Err(error().into())
        }
    } else if decimal_point_length == 2 {
        if (0..=15).contains(&digit_length) {
            Ok(PrecisionHelper::Prec2_3)
        } else {
            let error = || PrecisionCreationError::DigitLength(
                digit_length, decimal_point_length, include_negative, total_length
            );
            Err(error().into())
        }
    } else if decimal_point_length == 3 {
        if (0..=15).contains(&digit_length) {
            Ok(PrecisionHelper::Prec3_3)
        } else {
            let error = || PrecisionCreationError::DigitLength(
                digit_length, decimal_point_length, include_negative, total_length
            );
            Err(error().into())
        }
    } else if decimal_point_length == 6 {
        if (0..=12).contains(&digit_length) {
            Ok(PrecisionHelper::Prec6_6)
        } else {
            let error = || PrecisionCreationError::DigitLength(
                digit_length, decimal_point_length, include_negative, total_length
            );
            Err(error().into())
        }
    } else if decimal_point_length == 9 {
        if (0..=9).contains(&digit_length) {
            Ok(PrecisionHelper::Prec9_9)
        } else {
            let error = || PrecisionCreationError::DigitLength(
                digit_length, decimal_point_length, include_negative, total_length
            );
            Err(error().into())
        }
    } else {
        let error = || PrecisionCreationError::InvalidPrecision(
            digit_length, decimal_point_length, include_negative, total_length
        );
        Err(error().into())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_precision() -> Result<()> {
        let precision = get_precision_helper(NumReprCfg {
            digit_length: 15,
            decimal_point_length: 3,
            include_negative: false,
            total_length: 18,
        });

        assert!(precision.is_err());

        let precision = get_precision_helper(NumReprCfg {
            digit_length: 12,
            decimal_point_length: 0,
            include_negative: false, 
            total_length: 12,
        });

        assert!(precision.is_ok());
        assert_eq!(precision?, PrecisionHelper::Prec0_3);
        
        
        let precision = get_precision_helper(NumReprCfg {
            digit_length: 11,
            decimal_point_length: 2,
            include_negative: true, 
            total_length: 15,
        });
        
        assert!(precision.is_ok());
        assert_eq!(precision?, PrecisionHelper::Prec2_3);
        
        let precision = get_precision_helper(NumReprCfg {
            digit_length: 15,
            decimal_point_length: 3,
            include_negative: false,
            total_length: 19,
        });
        
        assert!(precision.is_ok());
        assert_eq!(precision?, PrecisionHelper::Prec3_3);

        let precision = get_precision_helper(NumReprCfg {
            digit_length: 8,
            decimal_point_length: 6,
            include_negative: true, 
            total_length: 16,
        });

        assert!(precision.is_ok());
        assert_eq!(precision?, PrecisionHelper::Prec6_6);


        let precision = get_precision_helper(NumReprCfg {
            digit_length: 18,
            decimal_point_length: 0,
            include_negative: false, 
            total_length: 18,
        });

        assert!(precision.is_ok());
        assert_eq!(precision?, PrecisionHelper::Prec0_0);

        let precision = get_precision_helper(NumReprCfg {
            digit_length: 9,
            decimal_point_length: 9,
            include_negative: false, 
            total_length: 19,
        });

        assert!(precision.is_ok(), "{:?}", precision?);
        assert_eq!(precision?, PrecisionHelper::Prec9_9);

        Ok(())

    }
}