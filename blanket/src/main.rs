use blanket::{Bird, CanFly, Car, Move, Tire, Wings};

fn main() {
    let car = Car {};
    let bird = Bird {};

    car.attach();
    car.move_distance();

    bird.swing();
    bird.fly();
}
