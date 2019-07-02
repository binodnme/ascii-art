extern crate image;


use std::env;
use dots::alg::{greyscale};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut filepath = "a.jpeg";
    if args.len() > 1 {
        filepath = &args[1];
    }        

    println!("{}", greyscale(filepath.to_string()))
}
