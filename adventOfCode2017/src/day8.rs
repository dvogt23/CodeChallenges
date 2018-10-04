use std::collections::HashMap;
use regex::Regex;

pub fn solve(input: Vec<String>) -> (i32, i32) {
    let mut highest = 0;
    let mut instructions: HashMap<String, i32> = HashMap::new();
    let re = Regex::new(r"^([^\s]+).([^\s]+).([^\s]+).if.([^\s]+).([^\s]+).(-?[0-9]\d*(\.\d+)?)").unwrap();

    for line in input {
        let cap = re.captures(&line[..]).unwrap();

        // parse instruction
        let name = cap.get(1).map_or("", |m| m.as_str());
        let op = cap.get(2).map_or("", |m| m.as_str());
        let value: i32 = cap.get(3).map_or(0, |m| m.as_str().parse().unwrap());

        // parse operation
        let ifname = cap.get(4).map_or("", |m| m.as_str());
        let ifop = cap.get(5).map_or("", |m| m.as_str());
        let ifvalue: i32 = cap.get(6).map_or(0, |m| m.as_str().parse().unwrap());

//        println!("{} {} {} if {} {} {}", name, op, value, ifname, ifop, ifvalue);

        if !instructions.contains_key(name) {
            instructions.insert(String::from(name), 0);
        }

        if !instructions.contains_key(ifname) {
            instructions.insert(String::from(ifname), 0);
        }

        if !instructions.contains_key(ifname) {
            instructions.insert(String::from(ifname), 0);
            continue;
        }

        let check = check_condition(&instructions, ifname, ifop, ifvalue);

        if check {
            match op {
                "inc" => {
                    *instructions.get_mut(name).unwrap() += value;
                    if instructions[name] > highest {
                        highest = instructions[name];
                    }
                },
                "dec" => {
                    *instructions.get_mut(name).unwrap() -= value;
                    if instructions[name] > highest {
                        highest = instructions[name];
                    }
                },
                _ => println!("Error in operation!")
            }
        }
    }

//    println!("{:?}", instructions);

    let greatest_value = instructions.values().max().unwrap();

//    println!("Greatest: {}\nHighest: {}", greatest_value, highest);

    (greatest_value.to_owned(), highest.to_owned())
}

fn check_condition(i: &HashMap<String, i32>, name: &str, op: &str, value: i32) -> bool {
    match op {
        ">" => if i.get(name).unwrap() > &value {return true},
        ">=" => if i.get(name).unwrap() >= &value {return true},
        "<" => if i.get(name).unwrap() < &value {return true},
        "<=" => if i.get(name).unwrap() <= &value {return true},
        "==" => if i.get(name).unwrap() == &value {return true},
        "!=" => if i.get(name).unwrap() != &value {return true},
        _ => println!("Operation not found!")
    }

    false
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(1, solve(vec![String::from("b inc 5 if a > 1"),
                                 String::from("a inc 1 if b < 5"),
                                 String::from("c dec -10 if a >= 1"),
                                 String::from("c inc -20 if c == 10")]).0);
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(10, solve(vec![String::from("b inc 5 if a > 1"),
                                 String::from("a inc 1 if b < 5"),
                                 String::from("c dec -10 if a >= 1"),
                                 String::from("c inc -20 if c == 10")]).1);
    }
}
