use std::str;
pub fn series(digits: &str, len: usize) -> Vec<&str> {
        (0..digits.len() + 1 - len)
                .map(|i| &digits[i..i + len])
                .collect()
}
