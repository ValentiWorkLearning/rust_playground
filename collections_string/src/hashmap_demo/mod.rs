use std::collections::HashMap;

pub mod hashmap_demo
{
    use std::collections::HashMap;

    pub fn run_hasmap_demo(){
        let mut scores = HashMap::new();
        scores.insert(String::from("Red"), 323);
        scores.insert(String::from("blue"), 123);


        for(key,value) in &scores{
            println!("key is {key}, value is : {value}");
        }
        let team_name = String::from("Red");
        let _red_result = scores.get(&team_name).copied().unwrap_or(0);

        scores.entry(String::from("Red")).or_insert(32);


        let text = "hello with hello worlds";
        let mut words_counter = HashMap::new();
        for word in text.split_whitespace(){
            let count = words_counter.entry(word).or_insert(0);
            *count+=1;
        }

        println!("{:?}",words_counter);
    }
}