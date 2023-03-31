use std::io::{self, Read};


fn main() {
    println!("Adivina el numero, Gana 2 pesos");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Fail");

    println!("Dijiste {guess}");

}
