// https://doc.rust-lang.org/book/appendix-03-derivable-traits.html
fn main() {
    {
        // Simple literal
        let _test_string = "124";
    }
    {
        // String type
        let _test_string  = String::from("1234");
    }
    {
        // mutable string
        let mut test_string = String::from("1234");
        test_string.push_str("Hello from!");
        println!("Test string is: {test_string}")
    }

    {
        // Simple copy
        let x = 5;
        let _y = x;
    }

    {
        // String copy
        let s1 = String::from("hello");
        let _s2 = s1.clone();

        println!("{}, world!", s1);
    }

    {
        // Ownership trasfer ex1
        let s= String::from("Hello worlds!");
        takes_ownership(s);

        let test_int = 42;
        makes_copy(test_int);

        
        println!("Attemp to use test_int after a call: {test_int}");
        // println!("Attemp to use s after a call: {s}");
    }

    {
        // Ownership trasfer ex2
        
        let _s1 = gives_ownership();
        let s2 = String::from("hello");

        let _s3  = takes_and_gives_back(s2);
    }

    {
        // String length with tuple
        let string_item = String::from("hello");
        let (string_back,length) = calculate_length(string_item);

        println!("The length of {string_back} is {length}");

    }
    {
        // String reference example
        let s1 = String::from("Hello");
        let string_length = calculate_length_ref_based(&s1);
        println!("The length of {s1} is {string_length}")
    }
    {
        // String modify reference example
        let mut s1 = String::from("Hello");
        try_modify(&mut s1);
        println!("Modified string: {s1}");

        // This compiles with the error:
        // 
        // 74 |         let ref1 = &mut s1;
        //    |                    ------- first mutable borrow occurs here
        // 75 |         let ref2 = &mut s1;
        //    |                    ^^^^^^^ second mutable borrow occurs here
        // let ref1 = &mut s1;
        // let ref2 = &mut s1;

        // println!("Ref1: {ref1}, ref2: {ref2}");
    }

    {
        // Can't take differrent refs:
        // This compiles with the error:

        //         89 |         let r1 = &s1;
        //    |                  --- immutable borrow occurs here
        // ...
        // 92 |         let mut r3 = &mut s1;
        //    |                      ^^^^^^^ mutable borrow occurs here
        // 93 |
        // 94 |         println!("R1:{r1}, R2:{r2}, R3: {r3}");
        //    |                       -- immutable borrow later used here

        // let mut s1 = String::from("HELLOOOO");
        // let r1 = &s1;
        // let r2 = &s1;

        // let mut r3 = &mut s1;

        // println!("R1:{r1}, R2:{r2}, R3: {r3}");



        let mut s = String::from("Something cool");
        let r1 = &s;
        let r2 =&s;
        println!("Refs are: {r1}, {r2}");

        let r3 = &mut s;
        r3.push_str("coooooooool");

        println!("After modification: {r3}");
    }
}


fn takes_ownership(some_string:String){
    println!("Ownneship was taken:{some_string}")
}

fn makes_copy(some_integer: i32){
    println!("makes copy:{some_integer}")
}

fn gives_ownership()->String{

    let some_string = String::from("A kind of a new string");
    some_string
}

fn takes_and_gives_back(test_string:String)->String{
    test_string
}


fn calculate_length(s:String)->(String, usize){
    let length = s.len();
    (s,length)
}

fn calculate_length_ref_based(s:&String)->usize{
    s.len()
}

fn try_modify(some_string:&mut String){
    some_string.push_str("hello world again!");
}


