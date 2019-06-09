use quicksilver::geom::{Vector, Rectangle};
use quicksilver::input::{Keyboard, Key};

struct Player<'a> {
    rect: Rectangle,
    velocity: Vector,
    keyboard: &'a Keyboard,
}


fn main() {
    println!("Hello, world!");
}
