

fn main() {
    let x:String = String::from("Owned");
    let x_len = reference(&x);
    println!("{x} {x_len}");

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

fn reference(x:&String)->usize{
    x.len()
}