pub trait Messnger {
    fn send(&self,msg:&str);
}


pub struct LimitTracker<'a, T:Messnger>{
    messenger:&'a T,
    value:usize,
    max:usize
}

use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


impl<'a,T> LimitTracker<'a,T> where T:Messnger
{
    pub fn new(messenger:&'a T, max:usize)->LimitTracker<'a,T>{
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value:usize){
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;


        if percentage_of_max >= 1.0{
            self.messenger.send("Error, you are over quota");
        }
        else if percentage_of_max >= 0.9{
            self.messenger.send("You've used 90% of quota");
        }
        else if percentage_of_max >= 0.75{
            self.messenger.send("You've used 75% of quota");
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger{
        sent_messages:RefCell<Vec<String>>
    }

    impl MockMessenger{
        fn new()->MockMessenger{
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messnger for MockMessenger{
        fn send(&self,msg:&str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn quota_almost_75(){
        let mock_messenger = MockMessenger::new();
        let mut limitr_tracker = LimitTracker::new(&mock_messenger,100);
        limitr_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(),1);
    }

    
    use crate::List::{Cons, Nil};

    fn test_list(){
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    }
}