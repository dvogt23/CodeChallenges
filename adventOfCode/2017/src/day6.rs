use std::collections::HashMap;

pub fn get_cycle(input: Vec<String>) -> (i32, i32) {
    let mut sum = 0;

    let mut banks: Vec<_> = input.first().unwrap().split_whitespace().filter_map(|b| b.parse::<u32>().ok()).collect();

    // for compare in loop
    let mut unique_set = HashMap::new();

    loop {
        let mut buffer = 0;
        let mut position = 0;

        // get max
        let  max: u32 = banks.iter().max().unwrap().clone();

        // get position of max value and set buffer value
        for i in 0..banks.len() {
            if banks[i] == max {
                buffer = banks[i];
                banks[i] = 0;
                position = i;
                break;
            }
        }

        while buffer > 0 {
            position = (position + 1) % banks.len();
            banks[position] += 1;
            buffer -= 1;
        }

        sum += 1;

        // check if already seen
        if unique_set.contains_key(&banks) {
            return (sum, (sum - unique_set.get(&banks).unwrap()));
        } else {
            unique_set.insert(banks.clone(), sum);
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(5, get_cycle(vec![String::from("0	2	7	0")]).0);
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(4, get_cycle(vec![String::from("0	2	7	0")]).1);
    }
}
