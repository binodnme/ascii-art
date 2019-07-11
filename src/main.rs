extern crate image;

use ascii_art::alg::greyscale;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut filepath = "a.jpeg";
    if args.len() > 1 {
        filepath = &args[1];
    }

    println!("{}", greyscale(&filepath.to_string()))
}
