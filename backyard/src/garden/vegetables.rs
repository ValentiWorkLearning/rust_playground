
pub struct Asparagus{
    tree_name: String
}

impl Asparagus{
    pub fn print_tree(&self){
        println!("Self tree {}",self.tree_name);
    }
}

pub fn make_plant(some_name:&str)->Asparagus{
    Asparagus{
        tree_name:String::from(some_name)
    }
}