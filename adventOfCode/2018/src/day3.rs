use regex::Regex;
use std::collections::{HashMap};

#[derive(Debug)]
struct Entry {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Entry {
    fn from_string(line: &str) -> Entry {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let caps = re.captures(line).unwrap();

        let id: u32 = caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
        let x: u32 = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap());
        let y: u32 = caps.get(3).map_or(0, |m| m.as_str().parse().unwrap());
        let width: u32 = caps.get(4).map_or(0, |m| m.as_str().parse().unwrap());
        let height: u32 = caps.get(5).map_or(0, |m| m.as_str().parse().unwrap());

        Entry { id, x, y, width, height }
    }
}

pub fn main(input: Vec<String>) -> (usize, u32) {
    let mut grid: HashMap<(u32, u32), i8> = HashMap::new();
    let mut entries: Vec<Entry> = vec![];

    for line in input.iter() {
        let entry = Entry::from_string(&line);
        for i in entry.x + 1..entry.x + entry.width + 1 {
            for j in entry.y + 1..entry.y + entry.height + 1 {
                if !grid.contains_key(&(i, j)) {
                    grid.insert((i, j), 1);
                } else {
                    if let Some(x) = grid.get_mut(&(i, j)) {
                        *x += 1;
                    }
                }
            }
        }
        entries.push(entry);
    }

    (part1(&grid), part2(&grid, &entries))
}

fn part1(grid: &HashMap<(u32, u32), i8>) -> usize {
    grid.iter().filter(|(_, v)| **v >= 2).count()
}

fn part2(grid: &HashMap<(u32, u32), i8>, entries: &Vec<Entry>) -> u32 {
    'outer: for entry in entries {
        for i in entry.x + 1..entry.x + entry.width + 1 {
            for j in entry.y + 1..entry.y + entry.height + 1 {
                if *grid.get(&(i, j)).unwrap() != 1 {
                    continue 'outer
                }
            }
        }
        return entry.id
    }
    panic!("not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(
            4,
            main(vec![
                "#1 @ 1,3: 4x4".to_string(),
                "#2 @ 3,1: 4x4".to_string(),
                "#3 @ 5,5: 2x2".to_string(),
            ]).0
        );
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(
            3,
            main(vec![
                "#1 @ 1,3: 4x4".to_string(),
                "#2 @ 3,1: 4x4".to_string(),
                "#3 @ 5,5: 2x2".to_string(),
            ]).1
        );
    }
}
