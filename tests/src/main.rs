

fn main() {
    let x:&str = "Owned";
    ownership_test(x);
    println!("{x}");

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

fn ownership_test(some_string:&str){
    println!("{some_string}");
}