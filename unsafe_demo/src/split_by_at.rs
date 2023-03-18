use std::slice;

use crate::{demo_trait};


pub struct SplitAtMut{}

fn split_at_mut_simple(values: &mut[i32], mid:usize)->(&mut [i32], &mut[i32]){
    let array_length = values.len();
    assert!(mid <=array_length);
    // should be implemented in unsafe scope because mutable borrow occurs twice
    //(&mut values[..mid],&mut values[mid..])

    let array_ptr = values.as_mut_ptr();
    unsafe{
        (
            slice::from_raw_parts_mut(array_ptr,mid),
            slice::from_raw_parts_mut(array_ptr.add(mid), array_length-mid)
        )
    }
}

impl demo_trait::Demo for SplitAtMut{

    fn run_demo(&self){

        let mut vector = vec![1,2,3,4,5,6];
        let r = &mut vector[..];
        let (v1,v2) = split_at_mut_simple(r,3);

        assert_eq!(v1,&mut [1,2,3]);
        assert_eq!(v2,&mut [4,5,6]);


        self.lets_crash();
    }
}

impl SplitAtMut
{
    fn lets_crash(&self){
        let address:i64 = 0xdeadbeef;
        let r = address as *mut i32;
        let values: &[i32] = unsafe{slice::from_raw_parts_mut(r, 100)};
        for &item in values.iter(){
            println!("Item from deadbeef memory is:{}",item);
        }
    }
}