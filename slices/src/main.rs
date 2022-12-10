fn main() {
    let mut string_base = String::from("Hello, world!");

    let first_word_res = first_word(&string_base);

    string_base.clear();


    {
        // slice sample
        let s = String::from("Hello world");
        let hello = &s[0..5];
        let world = &s[6..11];

        let from_beginning = &s[..5];

        let str_length = s.len();
        let full_string = &s[..str_length];
        let full_string_2 = &s[..];
    }
    {
        let s = String::from("hello world");
        let first_word = first_word_slice_based(&s);

        // C++ allows this, lol
        // s.clear();

        println!("First word is:{first_word}");
    }

    {
        let literal_string  = "hello world";
        let slice = &literal_string[..];
        let first_word = first_word_slice_based(&literal_string);
    }

    {

        let array_test = [1,2,3,4,5,6,7];
        let slice = &array_test[1..4];

        assert_eq!(slice, &[2,3,4]);
    }
}



fn first_word(s:&String)->usize
{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i
        }
    }

    s.len()
}

fn first_word_slice_based(s:&str)->&str{
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i]
        }
    }
    &s[..]
}

