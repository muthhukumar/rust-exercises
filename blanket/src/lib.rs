pub struct Bird {}
pub struct Car {}

pub trait CanFly {
    fn fly(&self) {
        println!("Flying");
    }
}

pub trait Wings {
    fn swing(&self) {
        println!("Swing the winds to fly");
    }
}

pub trait Tire {
    fn attach(&self) {
        println!("Moving");
    }
}

pub trait Move {
    fn move_distance(&self) {
        println!("Moving distance");
    }
}

impl Tire for Car {
    fn attach(&self) {
        println!("Attach tires to car")
    }
}

impl Wings for Bird {
    fn swing(&self) {
        println!("swinging the wings");
    }
}

impl<T: Tire> Move for T {
    fn move_distance(&self) {
        println!("Moving distance because we have tires");
    }
}

impl<T: Wings> CanFly for T {
    fn fly(&self) {
        println!("flying");
    }
}
