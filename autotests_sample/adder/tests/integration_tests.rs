use adder;
mod common;

#[test]
fn integration_adder(){
    assert_eq!(adder::add_two(1),42)
}