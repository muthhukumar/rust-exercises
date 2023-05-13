enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum Number {
    One,
    Two,
    Three,
}

// * is the glob operator - global?
use Number::*;
use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;

    let green = TrafficLight::Green;

    let one = One;
    let two = Two;
    let three = Three;
}
