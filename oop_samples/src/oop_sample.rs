use std::{rc::Rc, str::FromStr};

pub trait Widget {
    fn draw(&self);
}


// pub struct Screen<T:Widget>{
//     pub components: Vec<Rc<T>>
// }

// impl<T> Screen<T> where T:Widget,
// {

//     pub fn run(&self){
//         for component in self.components.iter(){
//             component.draw();
//         }
//     }
// }

pub struct Screen{
    pub components: Vec<Rc<dyn Widget>>
}

impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }

    pub fn add_component(&mut self, component:Rc<dyn Widget>){
        self.components.push(component);
    }
}

struct Button{
    pub width:i32,
    pub height:i32,
    pub label: String
}

impl Widget for Button{
    fn draw(&self) {
        println!("Drawing button!");
    }
}

struct Spinbox{
    pub width:i32,
    pub height:i32,
    pub choice:Vec<String>
}

impl Widget for Spinbox {
    fn draw(&self) {
        println!("Drawing spinbox!");
    }
}
pub fn run_sample(){
    let mut main_screen = Screen{components:Vec::new()};
    let button = Rc::new(Button{width:10,height:10,label:String::from("Hello")});
    let spinbox = Rc::new(Spinbox{width:20,height:30,choice:vec![String::from("Hello"),String::from("World"),String::from("Here")]});

    main_screen.add_component(button);
    main_screen.add_component(spinbox);
    main_screen.run();
}