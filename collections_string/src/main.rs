
pub mod vector_demo;
pub mod strings_demo;
pub mod hashmap_demo;

use hashmap_demo::hashmap_demo::run_hasmap_demo;

fn main() {
    vector_demo::vec_demo::prepare_vec_test();
    strings_demo::strings_demo::run_strings_demo();
    run_hasmap_demo();
}
