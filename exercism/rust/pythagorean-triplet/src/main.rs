mod lib;

use lib::find;
fn main() {
    println!("p: {}", find().unwrap());
}
