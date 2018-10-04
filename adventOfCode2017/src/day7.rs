use regex::Regex;
use std::collections::HashMap;

struct Program {
    name: String,
    id: u32,
    childs: Vec<String>,
}


pub fn part1(input: Vec<String>) -> String {
    let mut program_list = HashMap::new();

    for line in input {
        let splitted = line.split(" -> ").collect::<Vec<&str>>();
        let mut program_part: &str = "";
        let mut child_part: Vec<String> = vec![];

        let reg_program = Regex::new(r"^(\w+).\W(\w+)").unwrap();

        program_part = splitted[0];
        if splitted.len() == 2 {
            child_part = splitted[1].split(", ").map(|w| w.to_string()).collect();
        }

        let group = reg_program.captures(program_part).unwrap();
        let prg = Program {
            name: group[1].to_string(),
            id: group[2].parse::<u32>().unwrap(),
            childs: child_part,
        };

        program_list.insert(prg.name.to_owned(), prg);
    }

    let mut length = 0;
    let mut root = "";
    for (name, program) in &program_list {
        let temp = get_path_length(program_list.get(name).unwrap(), &program_list, 0);
        if temp > length {
            root = &name;
            length = temp;
        }
    }

//    let root_childs = (program_list.get(root).unwrap().childs as Vec<String>)
//        .iter()
//        .filter_map(|p| (p, program_list.get(p).unwrap()))
//        .collect::<HashMap<String, Program>>();

    for (name, program) in &program_list {
            let w = get_weight(program_list.get(name).unwrap(), &program_list, 0) + program.id;
            if w != 0 {
                println!("weight: {}", w);
            }
    }
    root.to_string()
}

// get recrusive the length for some program
fn get_path_length(program: &Program, list: &HashMap<String, Program>, mut l: u16) -> u16 {
    if program.childs.len() == 0 {
        return 0;
    }

    let mut steps = 0;
    for c in &list.get(&program.name).unwrap().childs {
        steps += get_path_length(list.get(c).unwrap(), &list, l + 1);
    }

    l + steps
}

fn get_weight(program: &Program, list: &HashMap<String, Program>, mut l: u32) -> u32 {
    let mut steps: u32 = 0;
    for child in &list.get(&program.name).unwrap().childs {
        steps += get_weight(list.get(child).unwrap(), &list, l + list.get(child).unwrap().id)
    }

    l + steps
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(
            "tknk",
            part1(vec![
                String::from("pbga (66)"),
                String::from("xhth (57)"),
                String::from("ebii (61)"),
                String::from("havc (66)"),
                String::from("ktlj (57)"),
                String::from("fwft (72) -> ktlj, cntj, xhth"),
                String::from("qoyq (66)"),
                String::from("padx (45) -> pbga, havc, qoyq"),
                String::from("tknk (41) -> ugml, padx, fwft"),
                String::from("jptl (61)"),
                String::from("ugml (68) -> gyxo, ebii, jptl"),
                String::from("gyxo (61)"),
                String::from("cntj (57)"),
            ])
        );
    }
}
