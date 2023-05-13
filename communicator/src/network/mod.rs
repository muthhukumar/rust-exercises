pub mod server;

pub fn connect() {
    println!("network::connect");
    server::hi::hello_mod::connect();
}
