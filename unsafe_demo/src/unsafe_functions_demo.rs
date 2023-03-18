use crate::{demo_trait};


pub struct UnsafeFunctionCall{
}


unsafe fn dangerous(){
    println!("Booooooo....")
}

impl demo_trait::Demo for UnsafeFunctionCall{

    fn run_demo(&self) {
            unsafe{
                dangerous();
            }
    }
}