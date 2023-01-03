fn find_largest_number(list: &[i32])->&i32{
    let mut largest = &list[0];
    for number in list{
        if number > largest{
            largest = number;
        }
    }
    largest
}

struct Point<T>{
    x:T,
    y:T
}

struct MegaPoint<T,U,F>{
    x:T,
    y:U,
    z:F
}

struct SuperPoint<T>{
    x:T
}


impl<T> SuperPoint<T>{
    pub fn get_x(&self)->&T{
        &self.x
    }
}

enum Optional<T,E>{
    Some(T),
    ErrorValue(E),
    None
}


fn generic_struct_demo(){
    let integer_point = Point{ x:3,y:32};
    let float_point = Point{x:2.0, y:3.0};
    let strange_point = MegaPoint{x:2,y:2.0,z:'5'};

    let optional_test:Optional<i32, String> = Optional::Some(32);

    let super_point = SuperPoint{x:32};
    let x_value = super_point.get_x();
}

fn find_largest_number_generic<T:std::cmp::PartialOrd>(list: &[T])->&T{
    let mut largest = &list[0];
    for number in list{
        if number > largest{
            largest = number;
        }
    }
    largest
}


fn number_list_sample(){
    let number_list = vec![5,6,3,2,4,5,1];
    let largest = find_largest_number(&number_list);
    println!("The largest number is: {largest}");

    let second_number_list = vec![1,2,3,4,5,6,78,8,10];
    let second_largest = find_largest_number(&second_number_list);
    println!("The second largest number is {second_largest}");


    let char_list = vec!['1','2','3','4','5'];
    let largest_char = find_largest_number_generic(&char_list);
    println!("The largest char is: {largest_char}");
}

pub fn run_generics_demo(){
    number_list_sample();
    generic_struct_demo();
}