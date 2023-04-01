
struct Info {
    mail: String,
    name: String,
    age: u8,
}

fn main() {
    let information = strcture_test("example@gmail.com".to_string(),"TestName".to_string(),21);
    println!("{} {} {}",information.mail,information.name,information.age);
    let information2 = Info{
        age: 22,
        name: String::from("TestChangedName"),
        ..information
    };
    println!("{} {} {}",information2.mail,information2.name,information2.age);
}


// fn imp_return(x: f32, y:f32)-> f32 {
//     x / y
// }

// fn scoped(){
//     let s: &str= "no";

//     {
//         let mut s: String = String::from("yes");
//         s.push_str(" maybe");
//         println!("{s}");
//     }

//     println!("{s}");

// }

// fn ownership_test(some_string:&str){
//     println!("{some_string}");
// }

// fn reference(x:&String)->usize{
//     x.len()
// }
// fn slicer_test(x:String){

//     let sliced = &x[..3].len();
//     println!("{x} {sliced}");
// }

fn strcture_test(mail:String,name:String,age:u8)-> Info {
    Info{
        mail,
        name,
        age,
    }
    
}
