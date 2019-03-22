use std::collections::VecDeque;

fn get_closed(s: &char) -> &char {
    match s {
        '{' => &'}',
        '[' => &']',
        '(' => &')',
        _ => s,
    }
}

fn is_open(c: &char) -> bool {
    match c {
        '{' | '[' | '(' => true,
        _ => false,
    }
}

fn is_closed(c: &char) -> bool {
    match c {
        '}' | ']' | ')' => true,
        _ => false,
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut list = VecDeque::new();
    for c in string.chars() {
        if is_open(&c) {
            list.push_front(c);
        }

        if list.len() > 0 {
            if is_closed(&c) {
                if get_closed(&list[0]) == &c {
                    list.pop_front();
                } else {
                    return false;
                }
            }
        } else if is_closed(&c) {
            return false;
        }
    }
    list.len() == 0
}
