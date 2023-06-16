fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_new<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn main() {
    let result;
    let y = "string";

    {
        let x = String::from("testing");

        result = longest_new(x.as_str(), y);
    }

    println!("{}", result);

    let r;

    let x = 10;
    r = &x;

    println!("{r}");

    let longest_str = longest("testing", "new testing");

    let long_live_str = "testing";

    let result;

    {
        let short_live_str = "testiiiiiii";

        result = longest(long_live_str, short_live_str);
    }

    println!("{result}");

    println!("{longest_str}");

    this_works();
    this_not_works();
}

fn this_works() {
    let result;

    {
        let y = "world";
        result = y
    }

    println!("{}", result);
}

fn this_not_works() {
    let result;

    {
        let y = String::from("world");
        result = &y.as_str();
    }

    println!("{}", result);
}
