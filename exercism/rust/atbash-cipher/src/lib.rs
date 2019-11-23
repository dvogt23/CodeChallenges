const alpha: &str = "abcdefghijklmnopqrstuvwxyz";

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_alphabetic() || c.is_alphanumeric())
        .filter(|c| c.is_ascii())
        .map(|c| {
            if let Some(i) = alpha.char_indices().find(|f| f.1 == c) {
                alpha.chars().nth(25 - i.0).unwrap()
            } else {
                c
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            if let Some(i) = alpha.char_indices().find(|f| f.1 == c) {
                alpha.chars().nth(25 - i.0).unwrap()
            } else {
                c
            }
        })
        .collect::<String>()
}
