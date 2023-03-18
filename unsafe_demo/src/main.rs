
mod demo_trait;

mod unsafe_pointers_demo;
mod unsafe_functions_demo;
mod split_by_at;

use std::rc::Rc;

fn main() {
    let mut demo_container:Vec<Rc<dyn demo_trait::Demo>> = Vec::new();
    demo_container.push(Rc::new(unsafe_pointers_demo::UnsafePointersDemo{}));
    demo_container.push(Rc::new(unsafe_functions_demo::UnsafeFunctionCall{}));
    demo_container.push(Rc::new(split_by_at::SplitAtMut{}));

    for item in demo_container.iter(){
        item.run_demo();
    }
}
