use crate::{demo_trait};

pub struct UnsafePointersDemo
{
}

impl demo_trait::Demo for UnsafePointersDemo
{
    fn run_demo(&self){
        let mut num = 42;
        let pnum = &num as * const i32;
        let pmut_num = &mut num as *mut i32;
    
        let dummy_address:i64 = 0xDEADBEEF;
        let p_address = &dummy_address as * const i64;
    
        unsafe{
            println!("Unmutable ptr is: {}",*pnum);
            println!("Mutable pointer is: {}",*pmut_num);
        }
    }
}