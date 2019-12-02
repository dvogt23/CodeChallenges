extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

mod day1;
mod day2;

fn main() {
    // read args
    let args: Vec<_> = env::args().collect();

    // match args with/without input file
    match args.len() {
        2 => {
            let mut inputfile: String = "input/".to_owned();
            inputfile.push_str(&args[1][..]);
            run_day(args[1].to_string(), inputfile);
        }
        3 => {
            run_day(args[1].to_string(), args[2].to_string());
        }
        _ => println!("Need arguments: [dayX] {{optional: [inputfile]}}"),
    }
}

fn run_day(day: String, inputfile: String) {
    match &day[..] {
        "day1" => {
            let p1 = day1::part1(lines_from_file(inputfile.to_owned()));
            let p2 = day1::part2(lines_from_file(inputfile.to_owned()));
            println!("Part1: {}\nPart2: {}", p1, p2);
        }
        "day2" => {
            let p1 = day2::part1(lines_from_file(inputfile.to_owned()));
            let p2 = day2::part2(lines_from_file(inputfile.to_owned()));
            println!("Part1: {}\nPart2: {}", p1, p2);
        }
        _ => (),
    }
}

fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
