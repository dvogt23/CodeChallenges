pub fn part1(input: Vec<String>) -> f32 {
    input
        .into_iter()
        .map(|mass| {
            let mass = mass.parse::<f32>().unwrap();
            (mass / 3f32).floor() - 2f32
        })
        .sum()
}

pub fn part2(input: Vec<String>) -> f32 {
    input
        .into_iter()
        .map(|mass| {
            let mass = mass.parse::<f32>().unwrap();
            let mut tmp = (mass / 3f32).floor() - 2f32;
            let mut sum = tmp;
            loop {
                tmp = (tmp / 3f32).floor() - 2f32;
                if tmp <= 0f32 {
                    break;
                }
                sum += tmp;
            }
            sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {}

    #[test]
    fn part2_ex1() {
        assert_eq!(966f32, part2(vec!["1969".to_string()]))
    }
}
