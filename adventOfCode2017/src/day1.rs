pub fn part1(input: String) -> u32 {
    let mut numbers: Vec<u32> = vec![];
    for i in input.chars() {
        match i.to_digit(10) {
            Some(s) => numbers.push(s),
            None => continue,
        }
    }

    let mut sum = 0;
    for i in 1..numbers.len() {
        if numbers[i - 1] == numbers[i] {
            sum += numbers[i];
        }
    }

    if numbers[0] == numbers[numbers.len() - 1] {
        sum += numbers[0];
    }

    sum
}

pub fn part2(input: String) -> u32 {
    let mut numbers: Vec<u32> = vec![];
    for i in input.chars() {
        match i.to_digit(10) {
            Some(s) => numbers.push(s),
            None => continue,
        }
    }

    let mut sum = 0;
    for i in 0..numbers.len() {
        let index = (i + numbers.len() / 2) % numbers.len();
        if numbers[i] == numbers[index] {
            sum += numbers[i];
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(3, part1("1122".to_string()));
    }
    #[test]
    fn part1_ex2() {
        assert_eq!(4, part1("1111".to_string()));
    }
    #[test]
    fn part1_ex3() {
        assert_eq!(0, part1("1234".to_string()));
    }
    #[test]
    fn part1_ex4() {
        assert_eq!(9, part1("91212129".to_string()));
    }


    #[test]
    fn part2_ex1() {
        assert_eq!(6, part2("1212".to_string()));
    }
    #[test]
    fn part2_ex2() {
        assert_eq!(0, part2("1221".to_string()));
    }
    #[test]
    fn part2_ex3() {
        assert_eq!(4, part2("123425".to_string()));
    }
    #[test]
    fn part2_ex4() {
        assert_eq!(12, part2("123123".to_string()));
    }
    #[test]
    fn part2_ex5() {
        assert_eq!(4, part2("12131415".to_string()));
    }
}
