
mod demo_trait;

mod unsafe_pointers_demo;
mod unsafe_functions_demo;
mod unsafe_static_demo;
mod split_by_at;
mod macro_demo;

use std::rc::Rc;


type Callback = fn(i32)->i32;

fn add_one(val:i32)->i32
{
    val+1
}

fn do_twice(callback_action:Callback,arg:i32)->i32{
    callback_action(arg)+callback_action(arg)
}

enum Status {
    Value(u32),
    Stop,
}

fn closure_back()->Box<dyn Fn(i32)->i32>{
    Box::new(|x|x+1)
}
fn main() {

    let value = 40;
    let _modified = do_twice(add_one, value);
    let _list_of_statuses:Vec<Status> = (0u32..20).map(Status::Value).collect();

    let mut demo_container:Vec<Rc<dyn demo_trait::Demo>> = Vec::new();
    demo_container.push(Rc::new(unsafe_pointers_demo::UnsafePointersDemo{}));
    demo_container.push(Rc::new(unsafe_static_demo::StaticModificationCall{}));
    demo_container.push(Rc::new(split_by_at::SplitAtMut{}));
    demo_container.push(Rc::new(macro_demo::MacroDemo{}));
    demo_container.push(Rc::new(unsafe_functions_demo::UnsafeFunctionCall{}));

    for item in demo_container.iter(){
        item.run_demo();
    }
}
