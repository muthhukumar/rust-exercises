pub fn connect() {
    println!("network::server::connect");
}

pub mod hi {
    pub mod hello_mod {
        fn hellow() {}

        pub fn connect() {
            // hellow();
        }
    }

    const A: i32 = 10;

    // hello_mod::connect();
}
