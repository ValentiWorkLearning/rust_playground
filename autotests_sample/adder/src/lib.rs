pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn panic_test(){
        panic!("oh man");
    }

    #[test]
    fn rect_test_larger_can_hold_smaller(){
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn rect_test_smaller_cant_hold_larger(){
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_eq(){
        let x = 1;
        assert_eq!(add_two(x),3)
    }

    #[test]
    fn test_neq(){
        let x = 1;
        assert_ne!(add_two(x),32);
    }

    #[test]
    fn custom_assert_test(){
        let string_test = "something".to_string();
        let result = greeting(string_test.as_str());

        assert!(result.contains("Hello world"), "There is no Hello world in {}",result);
    }

    #[test]
    #[should_panic]
    fn has_panic_test(){
        let test_guess = Guess::new(101);
    }

    #[test]
    #[should_panic(expected = "value between 1 and 100")]
    fn test_simple_expected_panic(){
        let has_to_panic = Guess::new(0);
    }

    #[test]
    fn it_works_seems() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    #[ignore]
    fn ignored_test_case(){
        
    }
}
