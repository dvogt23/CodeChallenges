/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let alpha = "abcdefghijklmnopqrstuvwxyz";

    plain
        .to_lowercase()
        .char_indices()
        .filter(|c| !c.1.is_whitespace() && c.1.is_alphabetic() || c.1.is_alphanumeric())
        .filter(|c| c.1.is_ascii())
        .map(|c| {
            if let Some(i) = alpha.char_indices().find(|f| f.1 == c.1) {
                alpha.chars().nth(25 - i.0).unwrap()
            } else {
                c.1
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
    let alpha = "abcdefghijklmnopqrstuvwxyz";

    cipher
        .char_indices()
        .filter(|c| !c.1.is_whitespace())
        .map(|c| {
            if let Some(i) = alpha.char_indices().find(|f| f.1 == c.1) {
                alpha.chars().nth(25 - i.0).unwrap()
            } else {
                c.1
            }
        })
        .collect::<String>()
}
