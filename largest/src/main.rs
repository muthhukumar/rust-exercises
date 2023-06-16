use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let point = Point { x: 20, y: 40 };
    let float_point = Point { x: 20.3, y: 40.2 };

    point.x();
    point.y();
    float_point.f();
    mixup();
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn f(&self) -> f32 {
        self.x * self.y
    }
}

fn mixup() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let point1 = Point { x: 1, y: 10.0 };
    let point2 = Point { x: 'c', y: 10.0 };

    let point_3 = point1.mixup(point2);

    println!("{} {}", point_3.x, point_3.y);
}
