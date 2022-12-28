
enum IpAddrKind{
    V4,
    V6
}

struct IpAddr{
    kind:IpAddrKind,
    address:String
}

enum IpAddrKindDirect{
    V4(String), // associated with a string value
    V8(String)
}


// different enum types
enum IpAddrKindVarious {
    V4(u8,u8,u8,u8),
    V8(String)
}

// like a standard library
struct IpV4{

}

struct IpV6{

}

enum IpAddrKindStructureBased{
    V4(IpV4),
    V6(IpV6)
}

// Complex structure
enum Message {
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(Color)
}

struct Quit;
struct Move{
    x:i32,
    y:i32
}

//struct Write(String); // tuple-based struct

struct Color{
    r:u8,
    g:u8,
    b:u8
}

// method implementation for Message enum:

impl Message {
    fn call(&self){
        println!("Message is: hello world!")
    }
}

fn option_test(test_option:Option<String>){
    match test_option {
        Some(string)=>{
            println!("Hello with {string}")
        },
        None=>{
            println!("Hmm, hello from empty value")
        }
    }
}





#[derive(Debug)]
enum UnState{
    Alaska,
    Alabama
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quater(UnState)
}

fn coin_to_string(coin_value:Coin)->String{
    match coin_value {
        Coin::Penny=>String::from("1"),
        Coin::Nickel=>String::from("5"),
        Coin::Dime=>String::from("10"),
        Coin::Quater(state)=>{
            println!("State is: {:?}",state);
            String::from("25")
        }
    }
}

fn enum_test(){
    let coin= Coin::Quater(UnState::Alabama);
    let converted = coin_to_string(coin);
    println!("Coin is: {converted}");
}

fn add_hat(){}
fn remove_fancy_hat(){}
fn move_player(steps:i32){}
fn reroll(){}

fn dice_roll_test(){
    let roll =3;
    match roll {
        3=> add_hat(),
        7=>remove_fancy_hat(),
        others=>move_player(others)
    }

    match roll {
        3=>add_hat(),
        7=>remove_fancy_hat(),
        _=>reroll()
    }

    match roll{
        3=>add_hat(),
        7=>remove_fancy_hat(),
        _=>()
    }
}

fn if_let_test(){

    let max_value_config:u8 = 8;
    let config_value = Some(8u8);
    match config_value {
        Some(max_value) => {println!("Config max value is: {max_value}")}
        _=>()
    }

    let config_value_iflet=Some(8u8);
    if let Some(max_value_config)= config_value_iflet{
        println!("Config max  iflet value is: {max_value_config}");
    }
    else{
        println!("whoa")
    }

    
}

fn main() {

    let ip4 = IpAddrKind::V4;
    let ip6 = IpAddrKind::V6;

    let home = IpAddr{
        kind:IpAddrKind::V4,
        address:String::from("192.168.0.10")
    };

    let loopback = IpAddr{
        kind:IpAddrKind::V6,
        address:String::from("::1")
    };

    let direct_home = IpAddrKindDirect::V4((String::from("192.168.0.10")));
    let direct_loopback = IpAddrKindDirect::V8((String::from("::1")));

    let direct_various = IpAddrKindVarious::V4((192), (168), (0), (1));
    let direct_various_loopback = IpAddrKindVarious::V8((String::from("::1")));


    let message_test = Message::Write((String::from("Hello!")));
    message_test.call();

    option_test(Some(String::from("whoa")));
    option_test(None);

    let test_none:Option<String> = None;

    println!("Hello, world!");

    enum_test();
    dice_roll_test();
    if_let_test();
}
