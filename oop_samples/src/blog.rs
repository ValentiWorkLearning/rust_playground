trait State{
    fn request_review(self:Box<Self>)->Box<dyn State>;
    fn approve(self:Box<Self>)->Box<dyn State>;
    fn content<'a>(&self,post:&'a Post)->String{
        String::new()
    }
}

struct Draft{}
struct PendingReview{}

struct Published{}


impl State for Draft {
    fn request_review(self:Box<Self>)->Box<dyn State> {
        Box::new(PendingReview{})
    }
    fn approve(self:Box<Self>)->Box<dyn State> {
        self
    }
}

impl State for PendingReview{
    fn request_review(self:Box<Self>)->Box<dyn State> {
        self
    }
    fn approve(self:Box<Self>)->Box<dyn State> {
        Box::new(Published{})
    }
}

impl State for Published{
    fn request_review(self:Box<Self>)->Box<dyn State> {
        self
    }
    fn approve(self:Box<Self>)->Box<dyn State> {
        self
    }
    fn content<'a>(&self,post:&'a Post)->String {
        post.content.clone()
    }

}
pub struct Post{
    state:Option<Box<dyn State>>,
    content:String
}

impl Post{
    pub fn new() -> Post {
        Post{
            state:Some(Box::new(Draft{})),
            content:String::new()
        }
    }

    pub fn add_text(&mut self, text:&str){
        self.content.push_str(text);
    }

    pub fn content(&self)->String{
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self){
        if let Some(s) = self.state.take(){
            self.state = Some((s.approve()));
        }
    }
}