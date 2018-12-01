pub fn part1(input: Vec<String>) -> i32 {
    let mut sum = 0;

    let mut path: Vec<i32> = input.iter()
        .map(|m| m.parse::<i32>().unwrap())
        .collect();

    let size = path.len();
    let mut position = 0;
    while  position < size {
        let jump = path[position];
        path[position] += 1;
        position = ((position as i32) + jump) as usize;
        sum += 1;

        if position > size {
            break;
        }
    }

    sum
}

pub fn part2(input: Vec<String>) -> i32 {
    let mut sum = 0;

    let mut path: Vec<i32> = input.iter()
        .map(|m| m.parse::<i32>().unwrap())
        .collect();

    let size = path.len();
    let mut position = 0;
    while  position < size {
        let jump = path[position];
        match jump {
             j if j >= 3 => path[position] -= 1,
            _ => path[position] += 1,
        };
        position = ((position as i32) + jump) as usize;
        sum += 1;

        if position > size {
            break;
        }
    }

    sum
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(5, part1(vec![String::from("0"), String::from("3"),
                                 String::from("0"),
                                 String::from("1"),
                                 String::from("-3")]));
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(10, part2(vec![String::from("0"), String::from("3"),
                                 String::from("0"),
                                 String::from("1"),
                                 String::from("-3")]));
    }
}
