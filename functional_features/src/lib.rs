use core::num;
use std::{thread, vec};
use std::time::Duration;

#[derive(Debug,PartialEq,Copy,Clone)]
enum ShirtColor{
    Red,
    Blue
}

struct Invertory{
    shirts:Vec<ShirtColor>
}

impl Invertory{
    pub fn giveaway(&self,user_preference:Option<ShirtColor>)->ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self)->ShirtColor{
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts{
            match color {
                ShirtColor::Red=>num_red +=1,
                ShirtColor::Blue =>num_blue+=1
            }
        }

        if num_red > num_blue{
            ShirtColor::Red
        }else{
            ShirtColor::Blue
        }
    }
}

fn test_only_borrows(){

    let list = vec![1,2,3];
    println!("Before defining a closure: {:?}", list);

    let only_borrows = ||println!("From closure: {:?}",list);

    println!("Before calling a closure: {:?}",list);
    only_borrows();
    println!("After calling a closure: {:?}",list);
}

fn captured_and_mutated(){
    let mut list = vec![1,2,3];
    println!("Before defining a closure: {:?}", list);

    let mut borrows_mutation = || { list.push(7); println!("From closure: {:?}",list)};

    borrows_mutation();
    println!("After calling a closure: {:?}",list);
}

fn move_ownership(){
    let mut list = vec![1,2,3];
    println!("Before defining a closure: {:?}", list);

    thread::spawn(move || println!("from thread: {:?}",list)).join().unwrap();
}

// https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures-and-the-fn-traits
fn sorting_sample(){

    #[derive(Debug)]
    struct Rectangle{
        width:u32,
        height:u32
    }

    let mut list = [Rectangle{width:10,height:10},Rectangle{width:3,height:1}, Rectangle{width:8,height:1}];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r|{
        num_sort_operations+=1;
        r.width
        }
    );
    println!("{:#?}",list);
    println!("Num sort operations: {num_sort_operations}");
}


fn array_slicing_test(){
    let a:[u8;4] = [1,2,3,4];
    let len = a.iter().len();

    println!("TEST");
    let items = &a[0..2];
    for item in items {
        dbg!(item);
    }
    println!("TEST");
    let items2 = &a[0..=2];
    for item in items2 {
        dbg!(item);
    }
}

fn vec_iterators_demo(){
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for val in v1_iter{
        println!("Got value:{}",val);
    }
}

//sample implementation of Iterator:
pub trait IteratorSample {
    type Item;
    fn next(&mut self)->Option<Self::Item>;
}


pub fn functional_features_demo(){
    let store = Invertory{
        shirts:vec![ShirtColor::Blue,ShirtColor::Red,ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway = store.giveaway(user_pref1);

    println!("The user with preference {:?} gets {:?}",user_pref1,giveaway);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}",user_pref2,giveaway2);


    let expensive_closure = |num:u32|->u32{
        println!("Long calculation...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(5);

    let example_closure = |x|x;

    let s = example_closure(String::from("Hello"));

    //let n = example_closure(5);

    test_only_borrows();
    captured_and_mutated();
    move_ownership();
    sorting_sample();
    array_slicing_test();


    vec_iterators_demo();
}


#[derive(PartialEq,Debug)]
struct Shoe{
    size:u32,
    style:String,
}


fn shoes_in_size(shoes:Vec<Shoe>,shoe_size:u32)->Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn iterator_sample(){
        let vec = vec![1,2,3];
        let mut vec_iter = vec.iter();
        assert_eq!(vec_iter.next(),Some((&1)));
        assert_eq!(vec_iter.next(),Some((&2)));
        assert_eq!(vec_iter.next(),Some((&3)));
        assert_eq!(vec_iter.next(),None);
    }

    #[test]
    fn iterator_sum(){
        let v1 = vec![1,2,3];
        let v1_iter = v1.iter();

        let total:i32 = v1_iter.sum();
        assert_eq!(total,6);
    }

    #[test]
    fn iterator_map(){
        let v1 = vec![1,2,3];
        let v1_iter = v1.iter();
        let second_vec: Vec<_> = v1_iter.map(|x| x * 2 ).collect();
        assert_eq!(second_vec,vec![2,4,6]);

    }


    #[test]
    fn shoe_capture_close_demo(){
        let shoes = vec![
            Shoe{
                size:10,
                style:String::from("sneaker"),
            },
            Shoe{
                size:13,
                style:String::from("sandal"),
            },
            Shoe{
                size:10,
                style:String::from("boot")
            }
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(in_my_size,vec![Shoe{size:10,style:String::from("sneaker")}, Shoe{size:10,style:String::from("boot")}]);
    }
}