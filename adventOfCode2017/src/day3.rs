struct Position {
    x: i32,
    y: i32,
    value: i32,
}

pub fn part1() -> i32 {
    let sum = 0;

    sum
}

pub fn part2() -> i32 {
    let mut sum = 0;

    sum
}

fn get_distance(grid: &mut Vec<Position>, end: i32) -> i32 {
    let mut radius: i32 = 1;
    let mut x = 0;
    let mut y = 0;

    grid.push(Position{x: x, y: y, value: 1});
    for v in 1..end  {
        set_next_position(&mut x, &mut y, &mut radius);
        println!("x: {}, y: {}, r: {}", x, y, radius);

        grid.push(Position{x: 0, y: 0, value: v});
        println!("{:?}", v);
    }

    let mut p = grid.last().unwrap();

    p.value
}

fn set_next_position(x: &mut i32, y: &mut i32, r: &mut i32) {

    if x == y && x != r && x != &mut r.wrapping_neg() {
        *x = *x + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
//    fn part1_ex1() {
//        assert_eq!(0, part1("1".to_string()));
//    }
//
//    #[test]
//    fn part1_ex2() {
//        assert_eq!(3, part1("12".to_string()));
//    }
//
//    #[test]
//    fn part1_ex3() {
//        assert_eq!(2, part1("23".to_string()));
//    }
//
//    #[test]
//    fn part1_ex4() {
//        assert_eq!(31, part1("1024".to_string()));
//    }
}
