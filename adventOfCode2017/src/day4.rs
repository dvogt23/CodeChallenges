use itertools::Itertools;

pub fn part1(input: Vec<String>) -> i32 {
    let mut sum = 0;

    for pass in input {
        if check_passphrase1(pass) {
            sum += 1;
        }
    }

    sum
}

pub fn part2(input: Vec<String>) -> i32 {
    let mut sum = 0;

    for pass in input {
        if check_passphrase2(pass) {
            sum += 1;
        }
    }

    sum
}

fn check_passphrase1(pass: String) -> bool {
    let new_count = pass.split_whitespace().unique().count();
    new_count == pass.split_whitespace().count()
}

fn check_passphrase2(pass: String) -> bool {
    let mut words = pass.split_whitespace()
        .map(|w| w.chars().sorted().into_iter().collect())
        .collect::<Vec<String>>();

    words.sort();
    words.dedup();

    let word_count = pass.split_whitespace().collect::<Vec<&str>>().len();
    words.len() == word_count
}


// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(true, check_passphrase1(String::from("aa bb cc dd ee")));
    }

    #[test]
    fn part1_ex2() {
        assert_eq!(false, check_passphrase1(String::from("aa bb cc dd aa")));
    }

    #[test]
    fn part1_ex3() {
        assert_eq!(true, check_passphrase1(String::from("aa bb cc dd aaa")));
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(true, check_passphrase2(String::from("abcde fghij")));
    }

    #[test]
    fn part2_ex2() {
        assert_eq!(false, check_passphrase2(String::from("abcde xyz ecdab")));
    }

    #[test]
    fn part2_ex3() {
        assert_eq!(true, check_passphrase2(String::from("a ab abc abd abf abj")));
    }

    #[test]
    fn part2_ex4() {
        assert_eq!(true, check_passphrase2(String::from("iiii oiii ooii oooi oooo")));
    }
}
