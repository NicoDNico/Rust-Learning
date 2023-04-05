pub fn auth_user(username:String,password:&str)->String{
    
    if password == get_password(){
        return "test@email"
    }

    

}




//this function is supposed to be the getter password to check if the correct one was given.
// funny thing this is the first proper comment since i started learning rust.
// i was mostly following the book till now so it makes sense.
fn get_password(username:&str)->&'static str{
    "password123"
}