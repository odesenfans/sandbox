extern crate adder;

#[test]
fn add_two_itest() {
    assert_eq!(4, adder::add_two(2));
}
