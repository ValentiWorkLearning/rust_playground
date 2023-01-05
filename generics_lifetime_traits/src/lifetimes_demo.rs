
pub mod lifetime{
    use std::fmt::Display;

    fn longest<'a> (first:&'a str, second: &'a str) ->&'a str{
        if first.len() > second.len() {
            first
        } else {
            second
        }
    }

    fn longest_with_all_fonks<'a,T>(first:&'a str, second: &'a str, third: &'a T)->&'a str where T:Display,
    {
        println!("Third lifetimes rule:{}",third);
        if first.len() > second.len() {
            first
        } else {
            second
        }
    }
    struct LifetimeAnnotated<'a>{
        member:&'a str
    }

    impl <'a> LifetimeAnnotated<'a> {
        fn level(&self)->i32{
            3
        }

        //third lifetime rule
        fn level_test(&self, string:&'a str)->&str{
            println!("Third rule! {}",string);
            self.member
        }
    }
    pub fn run_demo(){
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("asdf");
            let longest_string = longest(string1.as_str(),string2.as_str());
            println!("The longest string is {}", longest_string);
        }


        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let demo = LifetimeAnnotated{
            member:first_sentence
        };
        demo.level();
        demo.level_test("something");

        //static lifetime sample
        let string_test: &'static str = "hello static world";
        let test_string = String::from("Whoa@");

        longest_with_all_fonks(novel.as_str(), &test_string.as_str(), &test_string);

    }
}