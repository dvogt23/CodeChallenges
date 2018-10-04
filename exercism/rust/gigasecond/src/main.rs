extern crate chrono;
mod lib;

use chrono::{DateTime, Utc};
use lib::after;

fn main() {
    println!("res: {}", after(Utc::now()));
}

