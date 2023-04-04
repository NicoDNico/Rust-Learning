#[derive(Debug)]
mod Adder {

    mod negative{
        fn rest_number(x:i32, y:i32)-> i32{
        x-y
    }
    
        fn rest_letters(){}
    }

    mod positive{

        fn add_numbers(x:i32, y:i32)->i32{
            
        x+y

        }

        fn add_letters(){}
    }
}