use crate::{demo_trait};



pub struct StaticModificationCall
{
}

static mut HELLO_WORLD:&str = "Hello world";

impl demo_trait::Demo for StaticModificationCall{
    fn run_demo(&self) {
        unsafe{
            println!("String is:{}",HELLO_WORLD);
            HELLO_WORLD.replace("world", "omega world");
            println!("After modification:{}",HELLO_WORLD);
        }
    }
}