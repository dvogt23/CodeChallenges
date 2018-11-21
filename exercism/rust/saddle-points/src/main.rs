mod lib;

use lib::find_saddle_points;

fn main() {
    let input = vec![
        vec![4, 5, 4],
        vec![3, 5, 5],
        vec![1, 5, 4],
    ];

    println!("{:?}", find_saddle_points(&input));
}