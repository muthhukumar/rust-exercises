use adder_new;
mod common;

#[test]
fn integration_test() {
    common::setup();
    assert_eq!(10, adder_new::add_two(8))
}
