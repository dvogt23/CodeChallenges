use std::collections::HashMap;

pub fn part1(input: Vec<String>) -> usize {
    let (mut twos, mut threes): (usize, usize) = (0, 0);

    for line in input.iter() {
        let mut rep: HashMap<char, usize> = HashMap::new();
        line.chars().into_iter().for_each(|c| {
            if rep.contains_key(&c) {
                *rep.get_mut(&c).unwrap() += 1;
            } else {
                rep.insert(c, 1);
            }
        });

        if rep.values().any(|v| v == &2) {
            twos += 1;
        }
        if rep.values().any(|v| v == &3) {
            threes += 1;
        }
    }

    twos * threes
}

pub fn part2(input: Vec<String>) -> String {
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let mut diff: (u8, String) = (0, "".to_string());
            for k in 0..input[i].chars().count() {
                if input[i].chars().nth(k) != input[j].chars().nth(k) {
                    diff.0 += 1;
                } else {
                    diff.1.push(input[i].chars().nth(k).unwrap())
                }
            }

            if diff.0 == 1 {
                return diff.1
            }
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(
            12,
            part1(vec![
                "abcdef".to_string(),
                "bababc".to_string(),
                "abbcde".to_string(),
                "abcccd".to_string(),
                "aabcdd".to_string(),
                "abcdee".to_string(),
                "ababab".to_string(),
            ])
        );
    }

    #[test]
    fn part2_ex1() {
        assert_eq!("fgij", part2(vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "fguij".to_string(),
            "axcye".to_string(),
            "wvxyz".to_string(),
        ]));
    }
}
