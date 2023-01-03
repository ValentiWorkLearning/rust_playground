use crate::traits_demo::Summary;

pub struct NewsFromTwitter{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String
}
impl Summary for NewsFromTwitter{
    fn summarize(&self)->String{
        format!("{} by {} from {}",self.headline, self.author,self.location)
    }

    fn whoami(&self)->String {
        format!("I'm {}",self.author)
    }
}


