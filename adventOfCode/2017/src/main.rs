extern crate num;
extern crate itertools;
extern crate regex;

use std::env;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

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
    let filecontent = get_file_content(&*inputfile);

    match &day[..] {
        "day1" => {
            let p1 = day1::part1(filecontent.to_owned());
            let p2 = day1::part2(filecontent.to_owned());
            println!("Part1: {}\nPart2: {}", p1, p2);
        }
        "day2" => {
            let p1 = day2::part1(filecontent.to_owned());
            let p2 = day2::part2(filecontent.to_owned());
            println!("Part1: {}\nPart2: {}", p1, p2);
        }
        "day3" => {
            let p1 = day3::part1();
            let p2 = day3::part2();
            println!("Part1: {}\nPart2: {}", p1, p2);
        }
        "day4" => {
            let p1 = day4::part1(lines_from_file(inputfile.to_owned()));
            let p2 = day4::part2(lines_from_file(inputfile.to_owned()));
            println!("Part1: {}\nPart2: {}", p1, p2);
        }
        "day5" => {
            let p1 = day5::part1(lines_from_file(inputfile.to_owned()));
            let p2 = day5::part2(lines_from_file(inputfile.to_owned()));
            println!("Part1: {}\nPart2: {}", p1, p2);
        }
        "day6" => {
            let p1 = day6::get_cycle(lines_from_file(inputfile.to_owned())).0;
            let p2 = day6::get_cycle(lines_from_file(inputfile.to_owned())).1;
            println!("Part1: {}\nPart2: {}", p1, p2);
        }
        "day7" => {
            let p1 = day7::part1(lines_from_file(inputfile.to_owned()));
            //            let p2 = day7::part1(lines_from_file(inputfile.to_owned())).1;
            println!("Part1: {}\nPart2: ", p1);
        }
        "day8" => {
            let p1 = day8::solve(lines_from_file(inputfile.to_owned())).0;
            let p2 = day8::solve(lines_from_file(inputfile.to_owned())).1;
            println!("Part1: {}\nPart2: {}", p1, p2);
        }
        _ => (),
    }
}

fn get_file_content(path: &str) -> String {

    // Create a path to the desired file
    let path = Path::new(path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => return s,
    };
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
