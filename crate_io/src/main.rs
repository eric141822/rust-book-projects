use crate_io::mix;
use crate_io::PrimaryColor;
fn main() {
    let red = PrimaryColor::Red;
    let blue = PrimaryColor::Blue;
    let purple = mix(red, blue);
    println!("red + blue = {:?}", purple);
}
