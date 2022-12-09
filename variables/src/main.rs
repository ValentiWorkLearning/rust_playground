
fn main() {
    integers_print();
    float_print();
    various_print();
    tuples_print();
    arrays_print();
    something_defined_below_main();

    printer_dummy_vars(123);
    printer_dummy_vars(1234);

    super_dummy_printer(1234, false);

    expression_test();

    let mega_return = mega_returner();
    print!("result of mega-returner is: {mega_return}");

    let shit_happens = match mega_returner_v2(42){
        Some(_some_value)=>42,
        None=>24
    };
    println!("Shit happens result: {shit_happens}");


    if_condition_test();
    loop_test();
    label_loop_test();
    whlie_test();
    array_it_test();
    funny_numbers();

    for fib_it in 0..5{
        let fib_result = fib_counter(fib_it);
        println!("Nth fib member is: {fib_result}")
    }

    const CELSIUS_VALUE:u32=30;
    let converted_temp = cel_far_converter(CELSIUS_VALUE as f32);
    println!("Converted temp is: {converted_temp} °F");

}


fn integers_print(){
    let mut x = 5;
    println!("Hello x: {x}!");

    const _THREE_HOURS_IN_SECONDS:u32 = 60*60*3;
    
    {
        // Integers

        let x= x*2;
        println!("X inner scope: {x}");

        let spaces = "     ";
        let spaces = spaces.len();

        println!("spaces:{}", spaces);


        let test_bin = 0b0010;
        println!("test_bin is: {test_bin}");

        let byte_test = b'A';
        println!("byte_test: {byte_test}");


        let mut overflow_test:u8 = 0b1111_1110;
        println!("overflow_test start: {overflow_test}");
        overflow_test = overflow_test.wrapping_add(1);
        println!("overflow_test wrapping_add: {overflow_test}");

        let checked_add = overflow_test.checked_add(1);
        match checked_add{
             Some(added) => println!("Add result:{added}"),
             None => println!("Can't execute add operation")
        };
        println!("overflow_test wrapping_add: {overflow_test}");
    }

    x = 6;
    print!("Hello x: {x}");

}

fn float_print(){
    // Floats
    let _float_test = 2.0;
    let _second_float:f32 = 3.0;
}

fn various_print(){
     // Various
     let _test_bool = true;
     let _test_char = 'f';
     let test_utf8 = '😻';
     println!("UTF test: {test_utf8}");
}


fn tuples_print(){
    // Tuples
    let tuple_test = (1,2,3);
    let (x,y,z) = tuple_test;
    println!("Tuple is: {x} {y} {z}");

    let tuple_again = (1,3,4);
    let first_element = tuple_again.0;
    println!("Elements access: {first_element}");
}

fn arrays_print(){
    // Arrays 
    let array_test = [1,2,3,4,5,6];
    let last_item = match array_test.get(array_test.len() -1 ) {
        Some(item_node)=>item_node,
        None=> panic!("Something is wrong")
    };

    println!("Array item access {last_item}");
}

fn printer_dummy_vars(to_print:i32){
    println!("Something to print is: {to_print}");
}

fn super_dummy_printer(to_super_print:i32, to_second: bool){
    println!("Super printer: {to_super_print}, {to_second}");
}
fn something_defined_below_main(){
    println!("I'm staying below main");
}

fn expression_test(){
    let eval_result = {
        let x = 3;
        x+1
    };

    println!("Eval result is: {eval_result}");
    let bool_test = {false};

    println!("Seems we can do like this: {bool_test}")
}

fn mega_returner()->i32
{
    42
}

fn mega_returner_v2(_received_val:i32)->Option<i32>{
    return None;
}


fn if_condition_test(){
    let x = 5;
    if x< 10{
        println!("seems x too small");
    }
    else {
        println!("Whoa super x");
    }


    let _test_expression = if 2<5{2}else {3};
}

fn loop_test(){
    let mut counter = 0;

    let result = loop {
        counter+=1;

        if counter == 10{
            break counter * 2; 
        }
    };

    println!("The result is: {result}")
}

fn label_loop_test(){
    let mut top_counter = 0;

    'counting_loop:loop {
        println!("counter is: top_counter");

        let mut remaining = 10;

        'inner_loop: loop {
            println!(" remaining is: {remaining}");
             if remaining == 9{
                break 'inner_loop;
            }

            if top_counter == 2{
                break 'counting_loop;
            }
            remaining -=1;
        }
        top_counter +=1;
    }

    println!("End of the counting: {top_counter}")
}


fn whlie_test(){

    let mut counter = 10;

    while  counter != 0 {
        println!("while test: {counter}");
        counter -=1;
    }

    println!("While test done");
}

fn array_it_test(){
    let array = [1,2,3,4,5];

    for item in array{
        println!("item is: {item}");
    }
}

fn funny_numbers(){
    for it in 1..5{
        println!("number is: {it}")
    }

    for it in (0..10).rev(){
        println!("rev number is: {it}")
    }


    for it in (0..10).filter(|x|{ x % 2 != 0 })
    {
        println!("Filtred x is: {it}")
    }
}

fn fib_counter(fib_n:i32)->i32{
    if fib_n <= 1{
        return 1
    }
    else {
        return fib_counter(fib_n -1) + fib_counter(fib_n-2);
    }
}

fn cel_far_converter(temp:f32)->f32{
    temp*1.8 + 32.0
} 