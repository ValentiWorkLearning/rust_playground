use std::borrow::BorrowMut;
use std::{ops::Deref, borrow::Borrow};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
use crate::List::{Cons,Nil};

struct MyBox<T>(T);

impl <T> MyBox<T> {
 
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self) {
        println!("Drop is called for custom smart wrapper!");
    }
}
fn main() {

    let boxed_item = Box::new(5);
    println!("Boxed item is: {}",boxed_item);

    let list = Cons(Rc::new(RefCell::new(1)), Rc::new(Cons(Rc::new(RefCell::new(2)),Rc::new(Nil))));


    let value = 5;
    let pvalue = &value;

    println!("Value is:{}, pvalue is:{}",value,*pvalue);
    println!("Boxed value is:{}", *boxed_item);

    let my_box_instance = MyBox::new(5);

    println!("My box instance is:{}", *(my_box_instance.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);


    println!("Pointer c-tor");
    let custom_wrapper = CustomSmartPointer{ data:String::from("test_data")};
    println!("Pointer d-tor!");

    let custom_second_wrapper = CustomSmartPointer{data:String::from("Hello worlds!")};
    std::mem::drop(custom_second_wrapper);


    //Rc version
    {
        let a = Rc::new(Cons(Rc::new(RefCell::new(5)), Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil)))));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    }

    let a = Rc::new(Cons(Rc::new(RefCell::new(5)), Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));



    // RefCell sampple

}
