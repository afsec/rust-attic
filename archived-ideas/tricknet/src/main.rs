mod errors;
mod applets;

use applets::dispatcher;
use std::convert::From;
use std::process::exit;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");


fn main() {
    match dispatcher() {
        Ok(_) => {
            // dbg!(data);
            exit(0);
        }
        Err(error) => {
            // dbg!(&error);
            exit(i32::from(error));
        }
    }
}
