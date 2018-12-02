use std::collections::HashSet;

pub fn part1(input: Vec<String>) -> i32 {
    input.iter().map(|freq| freq.parse::<i32>().unwrap()).sum()
}

pub fn part2(input: Vec<String>) -> i32 {
    let mut seen = HashSet::new();
    seen.insert(0);

    let mut freq = 0;

    loop {
        for line in &input{
            freq += line.parse::<i32>().unwrap();
            if seen.contains(&freq) {
                return freq;
            }
            seen.insert(freq);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(
            3,
            part1(vec![
                "+1".to_string(),
                "-2".to_string(),
                "+3".to_string(),
                "+1".to_string()
            ])
        );
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(
            10,
            part2(vec![
                "+3".to_string(),
                "+3".to_string(),
                "+4".to_string(),
                "-2".to_string(),
                "-4".to_string()
            ])
        );
    }

    #[test]
    fn part2_ex2() {
        assert_eq!(
            5,
            part2(vec![
                "-6".to_string(),
                "+3".to_string(),
                "+8".to_string(),
                "+5".to_string(),
                "-6".to_string()
            ])
        );
    }
}
