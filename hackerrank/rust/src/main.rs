mod solutions;

use std::env;
use solutions::*;

fn main() {
    // read args
    let args: Vec<_> = env::args().collect();
    let input: String = args.iter().skip(2).cloned().map(|arg| arg + " ").collect::<String>();

    // match args with/without input file
    match args[1].as_str() {
        "1" => one::start(),
        "2" => two::start(input),
        _ => println!("solution not found!"),
    }
}

