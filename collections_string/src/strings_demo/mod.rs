pub mod strings_demo
{
    pub fn run_strings_demo(){
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("Appended string is: {s}");


        let string1 = "something".to_string();
        let string2 = " to concat".to_string();

        let concatenated = string1 + &string2;

        let string11 = "some".to_string();
        let string22 = "thing".to_string();

        let formatted_string = format!("{string11}-{string22}");
        println!("Formatted result is: {formatted_string}");

        let slice_example = "lorem ipsum bla bla";
        let sliced_string = &slice_example[0..5];
        println!("Sliced string is: {sliced_string}");
    }
}