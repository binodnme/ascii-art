extern crate image;

use dots::get_dots;

fn main() {
    let path = "./a.jpeg";
    println!("{}", get_dots(path.to_string()))
}
