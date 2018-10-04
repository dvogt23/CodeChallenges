pub fn part1(input: String) -> i32 {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        sum += get_diff(line);
    }

    sum
}

pub fn part2(input: String) -> i32 {
    let lines = input.split("\n");
    let mut sum = 0;
    for line in lines {
        sum += get_division(line);
    }

    sum
}

fn get_division(line: &str) -> i32 {
    let numbers = line.trim().split("\t");
    let mut div = 0;

    'out: for first in numbers.to_owned() {
        for second in numbers.to_owned() {
            let x = first.parse::<i32>().unwrap();
            let y = second.parse::<i32>().unwrap();
            if x != y && x % y == 0 {
                div = x/y;
                break 'out;
            }
        }
    }
    div
}

fn get_diff(line: &str) -> i32 {
    let numbers = line.trim().split("\t");
    let mut lowest = <i32>::max_value();
    let mut highest = 0;

    for n in numbers {
        let i = n.parse::<i32>().unwrap();
        if i > highest {
            highest = i;
        }
        if i < lowest {
            lowest = i;
        }
    }
    //println!("lowest: {}, highest: {}, diff: {}", lowest, highest, (highest - lowest));

    (highest - lowest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(18, part1("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8".to_string()));
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(9, part2("5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5".to_string()));
    }
}
