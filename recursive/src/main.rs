use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn print(x: &i32) {
    println!("{}", *x);
}

fn print_ln(x: &str) {
    println!("{}", x);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    let s = MyBox::new(String::from("testing"));

    print(&y);
    print_ln(&s);

    // println!("{}", y);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
