#[inline]
#[must_use]
pub fn contains_white_space(code: &[u8]) -> bool {
    code.contains(&b' ') || code.contains(&b'\t')
}

#[inline]
#[must_use]
pub fn all_white_space(code: &[u8]) -> bool {
    code.iter().all(|&c| c.is_ascii_whitespace())
}

#[inline]
#[must_use]
pub fn is_ascii(code: &[u8]) -> bool {
    code.iter().all(|&c| c.is_ascii())
}
