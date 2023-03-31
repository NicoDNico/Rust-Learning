use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivina el numero, Gana 2 pesos:");
    let secret = rand::thread_rng().gen_range(1..=100);
    loop{
    
    let mut guess = String::new();


    io::stdin()
        .read_line(&mut guess)
        .expect("Fail");

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    
    println!("Dijiste {guess}");

    match guess.cmp(&secret){
            Ordering::Equal =>{println!("Le pegaste");break;},
            Ordering::Greater=>println!("Muy alto"),
            Ordering::Less=>println!("Muy bajo"),
    }
}
}
