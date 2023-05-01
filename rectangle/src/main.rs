#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block
impl Rectangle {
    // here area is a method not a function
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_fit(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 40,
    };

    let square = Rectangle::square(30);

    // Here both are same
    println!("area of rectangle is {}", &rectangle.area()); // using method syntax to call the area function
    println!("area of rectangle is {}", rectangle.area()); // using method syntax to call the area function
    println!("does fit? {}", rectangle.can_fit(&rect2));
    println!("does fit? {}", rectangle.can_fit(&rect3));

    println!("square {:#?}", square);
}
