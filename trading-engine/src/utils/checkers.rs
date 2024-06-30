use anyhow::{Result, anyhow};

pub fn valid_isin_code_length(isin: &str) -> Result<bool> {
    match isin.len() == 12 {
        true => Ok(true),
        false => {
            let lazy_error = || anyhow!("ISIN code length error: {}", isin);
            Err( lazy_error() )
        }
    }
}

pub fn contains_white_space(code: &str) -> Result<bool> {
    match code.contains(' ') {
        true => {
            let lazy_error = || anyhow!("The value {} has a white space", code);
            Err( lazy_error() )
        },
        false => Ok(false)
    }
}

pub fn all_white_space(code: &str) -> Result<bool> {
    match code.chars().all(char::is_whitespace) {
        true => {
            let lazy_error = || anyhow!("The value {} is all white space", code);
            Err( lazy_error() )
        },
        false => Ok(false)
    }
}

pub fn is_ascii(code: &str) -> Result<bool> {
    match code.is_ascii() {
        true => Ok(true),
        false => {
            let lazy_error = || anyhow!("The value {} is not all alphabetic", code);
            Err( lazy_error() )
        }
    }
}