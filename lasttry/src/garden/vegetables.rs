#[derive(Debug)]
pub struct Asparagus {
    name: String,
}

impl Asparagus {
    fn test(&self)->String{
        self.name
    }
    
}

pub mod get_info{

    pub fn get_name()->String{
        let x = Asparagus{
            name:String::from("Papa"),
        };
    
        let y = x.test();
        y
    }
}