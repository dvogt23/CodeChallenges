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
            let mut mass = mass.parse::<f32>().unwrap();
            mass = get_fuel(mass);
            let mut sum = 0f32;
            while mass > 0f32 {
                sum += mass;
                mass = get_fuel(mass);
            }
            sum
        })
        .sum()
}

fn get_fuel(fuel: f32) -> f32 {
    (fuel / 3f32).floor() - 2f32
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
