// mod utils;
// use utils::database;
fn get_password()-> &'static str{
    "contrasena123"
}


pub fn modulador(password:&str){



        match password{
            get_password=> println!("verdadero"),
            _ => println!("Falso")
        }
    
}