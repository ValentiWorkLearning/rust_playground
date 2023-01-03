

pub mod panic_demo{
    use std::fs::{File, self};
    use std::io::{self, Read,ErrorKind};

    fn read_from_file()->Result<String,io::Error>{
        let username_file_result = std::fs::File::open("hello_user.txt");
        let mut username_file = match username_file_result {
            Ok(fd)=>fd,
            Err(error)=>{return Err(error)}
        };

        let mut username = String::new();
        match username_file.read_to_string(&mut username){
            Ok(_) => Ok(username),
            Err(e)=>Err(e),
        }
    }
    fn read_from_file_shorter()->Result<String,io::Error>{
        let mut username_file = std::fs::File::open("hello_user.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        let _result:Result<String, io::Error> = Ok(username);


        let mut username_second = String::new();
        File::open("hello_from_user.txt")?.read_to_string(&mut username_second)?;
        Ok(username_second)
    }

    // fn read_from_file_simpliest()->Result<String,io::Error>{
    //     fs::read_to_string("hello_from_user.txt")?;
    // }

    pub struct Guess{
        value:i32
    }

    impl Guess {
        pub fn new(value:i32)->Guess{
            if value < 1 || value > 100{
                panic!("guess creation is only possible when value is between 1 and 100. Passed value is: {value}");
            }

            Guess{
                value
            }
        }

        pub fn value(&self)->i32{
            self.value
        }
    }
    fn create_only_vaid_guess()
    {
        let _guess_valid = Guess::new(89);
        let _guess_invalid = Guess::new(-21);
    }
    pub fn run_panic_demo(){
        //panic!("crash and burn!");
        let hello_file_result = std::fs::File::open("hello.txt");
        let file = match hello_file_result {
            Ok(filename)=>{
                filename
            },
            Err(error)=>
            match error.kind() {
                ErrorKind::NotFound=>{
                    match std::fs::File::create("hello.txt") {
                        Ok(fd)=>fd,
                        Err(e)=>{
                            panic!("Failed to create the file: {:?}",e)
                        }
                    }
                },
                other_error=>{
                    panic!("Can`t open the file, error is: {:?}",other_error);
                }
            }
        };

        read_from_file();
        read_from_file_shorter();
        create_only_vaid_guess();
        //read_from_file_simpliest();

        let _hello_file_expect = std::fs::File::open("hello2.txt").expect("File should be available");

    }

}