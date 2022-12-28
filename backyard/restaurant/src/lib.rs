

// import something from standard library
use std::collections::HashMap;
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

    pub mod front_of_house {
    

    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String
    }

    impl Breakfast{
        pub fn summer(toast:&str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("cucumber")
            }
        }
    }

    // enum members are pub by default
    pub enum Appetizer{
        Soup,
        Salad
    }
    pub fn extra_function(){}

    pub mod hosting {
        pub fn add_to_waitlist()
        {
            // super for accessing the parent module
            super::extra_function();
        }

        fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    
}

//use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist;  

    pub fn eat_at_restaurant(){
        // absolute path with the crate
        crate::front_of_house::hosting::add_to_waitlist();

        // relative to module
        front_of_house::hosting::add_to_waitlist();

        let mut meal = crate::front_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");

        println!("I`d like a toast with {}", meal.toast);
        
        let apperite = crate::front_of_house::Appetizer::Soup;

        // with use
        {
            //hosting::add_to_waitlist();
            // full module function
            add_to_waitlist();
        }

        // collection
        {
            let mut map_test = HashMap::new();
            map_test.insert(1,2);
        }
    }

    // fn io_result()->IoResut<()>{}
    // fn fmt_result()->FmtResult{}

    pub fn test_result()
    {

    }
    pub use crate::front_of_house::serving;

    fn test_pub_use()
    {
        serving::take_order();
    }