fn main() {
    let _s = "testing"; // this cannot be mutated Because this uses stack memory
    let mut str = String::from("Hellow"); // But this can be mutated. This uses heap memory

    str.push_str("world");

    {
        let mut str_2 = String::from("hellow"); // Valid from this point onwards
        str_2.push_str("orld");
    }
    // str_2 is not valid from here. Because it is gone out of scope

    println!("{}", str);

    let x = 10;
    let y = x;

    let x_address = std::ptr::addr_of!(x);
    let y_address = std::ptr::addr_of!(y);

    println!("Address of x is {:?}", x_address);
    println!("Address of y is {:?}", y_address);

    let x_str = String::from("x");

    println!("Address of x_str is {:?}", std::ptr::addr_of!(x_str));
    let y_str = x_str; // String does not implement Copy trait because of that x_str is moved to the y_str

    // x_str - we will not be able to access x_str after this point. As it is moved to y_str

    println!("Address of y_str is {:?}", std::ptr::addr_of!(y_str));

    let str = String::from("Hello there");

    take_ownership(str); // here str is moved into the function. So using str beyond point is invalid

    let num = 100;

    take_copy(num); // here number has Copy trait so it got copied to the function

    let s1 = gives_ownership(); // the owner ship of the string from that function is passed to s1

    let s2 = String::from("s2"); // comes into scope

    let s3 = takes_and_transfers_ownership(s2); // ownership is transfered and again the s3 gets the ownership

    println!("s1, = {}, s3 = {}", s1, s3); // We can't use s2 here because the owner ship is passed to function and then to the s3

    reference_example();
}

fn take_ownership(str: String) {
    // str comes into scope
    println!("str {}", str);
} // str goes out of scope. here str is moved to the function when it is called so the backing memory is freed

fn take_copy(number: i32) {
    // Here numebr comes into scope
    println!("number is {}", number);
} // Here number goes out of scope. Nothing special happens here.

fn gives_ownership() -> String {
    let str = String::from("hello world");

    str
}

fn takes_and_transfers_ownership(str: String) -> String {
    str
}

fn reference_example() {
    let mut str = String::from("hello world");

    // Here we can still use the str because it is not moved to the function
    println!("{} length is {}", str, find_str_len(&str)); //  Here we are borrowing str referece and passing the referece to the function

    change_str(&mut str);

    println!("{} length is {}", str, find_str_len(&str)); // here the length is 18 because the str value is mutated by the chage_str function. Because it accepted the value as mutable reference
    dangling_reference();

    let mut s = String::from("hello world. This is muthukumar");

    let _word = first_word(&s);

    s.clear();
}

fn find_str_len(str: &String) -> usize {
    str.len()
}

fn change_str(str: &mut String) {
    str.push_str("hi here");
}

// Here we can't pass the reference. If we want to then we have to use static? lifetime?
// fn dangling_reference() -> &String{ // This will not work
fn dangling_reference() -> String {
    let str = String::from("test");

    str
    // &str
} // here the str goes out of scope so the memory for that is deallocated. So it points to nothing. so will prevent this for us.

fn first_word(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }
    &str[..]
}
