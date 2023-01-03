pub mod vec_demo
{

    fn dump_vector_content(vec_to_print:&Vec<i32>){
        println!("==VECTOR DUMP BEGIN==");
        for item in vec_to_print{
            print!("[{}]",item);
        }
        println!("\n==VECTOR DUMP END==");
    }

    fn access_vec_elements(){
        let test_vec = vec![1,2,3,4];

        let vec_element = &test_vec[2];
        println!("The third element is {vec_element}");

        let vec_element_opt = test_vec.get(2);
        match vec_element_opt {
            Some(item)=>{
                println!("Item is {item}");
            },
            None=>{
                println!("Ouch");
            }
        }
    }

    fn mutable_vec_demo(){
        let mut vec_test = vec![1,2,3,4,5];
        for item in &mut vec_test{
            *item *=2;
        }
        dump_vector_content(&vec_test);
    }
    #[derive(Debug)]
    enum SheetCell{
        Int(i32),
        Float(f64),
        Text(String)
    }

    fn enum_vec_demo(){
        let row = vec![SheetCell::Int(32), SheetCell::Float(0.56),SheetCell::Text(String::from("Hello world!"))];
        for item in row{
            match item {
                SheetCell::Float(_) =>{
                    dbg!(item);
                },
                _=>()
            }
        }
    }
    pub fn prepare_vec_test(){
        // with new c-tor
        let _v: Vec<i32> = Vec::new();   
        // with the vec! macro

        let _vec_macro = vec![1,2,3,4,5];


        //update vector content

        let mut vec_push = vec![1];
        vec_push.push(2);
        vec_push.push(3);

        for i in 1..10{
            vec_push.push(i);
        }

        dump_vector_content(&vec_push);
        access_vec_elements();
        mutable_vec_demo();
        enum_vec_demo();
    }
}