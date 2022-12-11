struct User{
    active:bool,
    username:String,
    email:String,
    sig_in_count:u64
}

fn create_user(username:String, email:String)->User{
    User { active: true, username, email, sig_in_count: 1 }
}


//tuple like structure
struct Point(i32,i32,i32);
struct Color(u8,u8,u8);


//unit like structure
struct AlwaysPrintable;


//lifetimes
struct UserLifetime<'a>{
    username : &'a str,
    email: &'a str
}

fn area(width:i32,height:i32) -> i64{
    let result = width*height;
    result as i64
}
#[derive(Debug)]
struct Rectangle{
    width:i32,
    height:i32
}

impl Rectangle{
    fn self_area(&self)->i64{
        let result = self.width * self.height;
        result as i64
    }
    fn has_width(&self)->bool{
        self.width >0
    }

    fn scale(&self, scale_factor:i32)->Rectangle{
        Rectangle { width:self.width*scale_factor, height: self.height*scale_factor }
    }

    //associated function

    fn square(size:i32)->Self{
        Self { width:size, height: size }
    }
}

// implementations can be separaredd from each other
impl Rectangle{
    fn can_hold(&self, other:&Rectangle)->bool{
        return  self.width > other.width && self.height> other.height;
    }
}

fn rect_area(rect:&Rectangle)->i64{
    let result = rect.width * rect.height;
    result as i64
}

fn compute_area_test(){
    let width = 42;
    let height = 42;
    let area = area(width, height);

    println!("Rectangle area is: {area} ");

    let rectangle = Rectangle
    {
        width:42,
        height:42
    };

    let rect_area = rect_area(&rectangle);
    println!("Rectangle area2 is: {rect_area}");

    //debug derive
    println!("Rectangle is: {:?}",rectangle);

    //verbose formatting
    println!("Rectangle versbose is: {:#?}",rectangle);

    //debug macro
    let rect_second = Rectangle{
        width: dbg!(42+3),
        height:42
    };

    dbg!(&rect_second);

    let self_area = rect_second.self_area();
    println!("Self rect area is: {self_area}");

    let _self_width = rect_second.self_area();


    let _scaled_rect = rect_second.scale(2);
}

fn can_hold_test(){
    let rect1 = Rectangle{width:0,height:0};
    let rect2 = Rectangle{width:41,height:41};
    let rect3 = Rectangle{width:42,height:42};

    let can_hold_2_in_1 = rect1.can_hold(&rect2);
    let can_hold_2_in_3 = rect3.can_hold(&rect2);

    dbg!(&can_hold_2_in_1);
    dbg!(&can_hold_2_in_3);
}

fn associated_test(){
    let base_rect = Rectangle{
        width:42,
        height:42
    };

    let square_rect = Rectangle::square(42);
    dbg!(square_rect);
}

fn main() {
    let mut user1 = User{
        email: String::from("test@mail.com"),
        username: String::from("Ivan Ivanenko"),
        active:true,
        sig_in_count:1
    };

    user1.email = String::from("some@mail.com");

    let user2 = create_user(String::from("some_user"), String::from("some@mail.com"));
    
    let _user3  = User{
        email: String::from("test@me.com"),
        ..user2
    };
    //username was moved during the creation of user3 from ..user2, the other values can be used
    //let user_2_access = user2.username;
    //println!("User2 is: {user_2_access}")

    let _point = Point(10,0,0);
    let _color = Color(255,0xff,0xff);

    let unit_structure = AlwaysPrintable;


    let user_lifetime = UserLifetime{
        username:"test",
        email:"some@me.com"
    };

    compute_area_test();
    can_hold_test();
    associated_test();

}
