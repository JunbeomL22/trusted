#[inline]
#[must_use]
pub fn valid_isin_code_length(isin: &str) -> bool {
    isin.len() == 12
}

#[inline]
#[must_use]
pub fn contains_white_space(code: &str) -> bool {
    code.contains(" ")
}

#[inline]
#[must_use]
pub fn all_white_space(code: &str) -> bool {
    code.chars().all(char::is_whitespace)
}

#[inline]
#[must_use]
pub fn is_ascii(code: &str) -> bool {
    code.is_ascii()
}