// use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = garden::vegetables::make_plant("test");
    plant.print_tree();
    println!("Hello, world!");
    restaurant::eat_at_restaurant();
}
