

pub struct Post
{
    content:String
}

pub struct DraftPost
{
    content:String
}

pub struct PendingReview{
    content:String
}

impl Post
{
    pub fn new()->DraftPost{
        DraftPost { content: String::from("") }
    }

    pub fn content(&self)-> &str{
        &self.content
    }
}

impl DraftPost{
    pub fn add_text(&mut self,new_text:&str){
        self.content.push_str(new_text);
    }

    pub fn request_review(&self)->PendingReview
    {
        println!("Switched to draft post, content is: {}",self.content);
        PendingReview{
            content:self.content.clone()
        }
    }
}

impl PendingReview{
    pub fn approve(&self)->Post{
        println!("Pending review approved, content is:{}",self.content);
        Post { content: self.content.clone() }
    }
}