


fn main() {

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
    //         println!("{some_string}");
    //     }
        
    //     fn reference(x:&String)->usize{
    //             x.len()
    //         }
    //         fn slicer_test(x:String){
                
    //                 let sliced = &x[..3].len();
    //                 println!("{x} {sliced}");
    //             }
                
                
    //  struct Info {
    //        mail: String,
    //        name: String,
    //        age: u8,
    //                 }
        //fn strcture_test(mail:String,name:String,age:u8)-> Info {
        //     Info{
        //         mail,
        //         name,
        //         age,
        //     }
        
        // }
    // #[derive(Debug)]
    // struct Info {
    //     user:String,
    //     kills: u32,
    //     deaths:u32,
    //     assists:u32,
    // }
    // impl Info{
    //     fn ratio(&self)-> f32{
    //         (self.kills + self.assists ) as f32 / self.deaths as f32 
    //     }
    //     fn is_better(&self,other_player:&Info )->bool{
    //         if self.ratio() > other_player.ratio() {
    //             return true
    //         }
    //         else {return false}
    //     }
    // }
        
    // fn info_builder(user:String , kills: u32 , deaths:u32, assists:u32) ->Info{
    //     Info{
    //         user,
    //         kills,
    //         deaths,
    //         assists,
    //     }
    // }
    
    // cant find another use for enums right now so im just using the ones in the book
    // #[derive(Debug)]
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // impl Message {
    //     fn call(&self) {
    //         println!("{:#?}",self);
    //     }
    // }