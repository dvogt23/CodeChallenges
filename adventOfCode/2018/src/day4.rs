use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Event {
    START { guard_id: u32 },
    ASLEEP,
    WAKEUP,
}

#[derive(Debug)]
struct Entry {
    year: u16,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
    event: Event,
}

impl Entry {
    fn from_str(s: &str) -> Entry {
        let re = Regex::new(r"(?m)\[([0-9]{4})-([0-9]{2})-([0-9]{2}) ([0-9]{2}):([0-9]{2})] (.*#(\d+)|.*([\s]*))")
            .unwrap();

        let caps = match re.captures(s) {
            None => panic!("unknown event"),
            Some(caps) => caps,
        };

        let event: Event =
            if let Some(x) = caps.get(7) {
                Event::START { guard_id: x.as_str().parse().unwrap_or_default() }
            } else if caps.get(6).map(|s| s.as_str()).unwrap() == "falls asleep" {
                Event::ASLEEP
            } else if caps.get(6).map(|s| s.as_str()).unwrap() == "wakes up".to_string() {
                Event::WAKEUP
            } else {
                panic!("unknown event");
            };

        let year: u16 = caps.get(1).map_or(0, |m| m.as_str().parse().unwrap_or_default());
        let month: u16 = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap_or_default());
        let day: u16 = caps.get(3).map_or(0, |m| m.as_str().parse().unwrap_or_default());
        let hour: u16 = caps.get(4).map_or(0, |m| m.as_str().parse().unwrap_or_default());
        let minute: u16 = caps.get(5).map_or(0, |m| m.as_str().parse().unwrap_or_default());

        Entry { year, month, day, hour, minute, event }
    }
}

pub fn part1(mut input: Vec<String>) -> u32 {
    let mut guards: HashMap<u32, u32> = HashMap::new();
    let mut cur_id: u32 = 0;
    let mut start_min: u16 = 0;
    input.sort();
    input.iter().for_each(|line| {
        let entry = Entry::from_str(&line);
        match entry.event {
            Event::START { guard_id } => {
                if entry.hour == 0 {
                    cur_id = guard_id;
                    guards.insert(cur_id, 1);
                }
            },
            Event::ASLEEP => {
                if cur_id != 0 {
                    start_min = entry.minute;
                }
            },
            Event::WAKEUP => {
                if cur_id != 0 {
                    *guards.get_mut(&cur_id).unwrap() += (entry.minute - start_min) as u32;
                }
            }
            _ => {}
        };
    });
    guards.iter()
        .max_by_key(|k| k.1).map(|g| g.0 * g.1).unwrap()
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
