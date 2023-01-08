use core::num;
use std::thread;
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
        height:u32,
    };

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
}