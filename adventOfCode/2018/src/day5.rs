pub fn part1(input: Vec<String>) -> u32 {
//    println!("{}, {} {}", 'j' as u64, 'J' as u64, 106-74);

    let mut ip: String = "tesStjjJjjsSsalskj".to_string();
    for (i, c) in ip.chars().enumerate() {
        if (c as u64) + 32 != (ip.chars().nth(i + 1).unwrap() as u64) ||
            (c as u64) - 32 != (ip.chars().nth(i + 1).unwrap() as u64){
            print!("{}", c)
        }
    }

    0
}

pub fn part2(input: Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(
            0,
            0
        );
    }
}
