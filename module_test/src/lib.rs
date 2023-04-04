#![allow(dead_code, unused_variables)]

pub struct Player{
    username:String,
    password:String,
    kills:u16,
    deaths:u16,
    assits:u16,
    rank:u8,
    games_played:u16,
    days_played:u16,
    status:bool,
}
impl Player{
    fn connect(self){
        self.status=true;
    }
}

mod database{
    #[derive(Debug)]
    pub struct Calculated_Stats{
        username:String,
        kda:f32,
        rank_per_day:f32,
        kills_per_game:u8,
        games_per_day:u16,
    }
    
    fn build_stats(user:&Player)->Calculated_Stats{
        
    }

    fn print_stats(stats:Calculated_Stats){
        println!("{:#?}",Calculated_Stats);
    }
}

mod utils{
    fn build_player(username,password,kills,deaths,assits,rank,games_played,days_played,status)->Player{
        user = new Player{
        username,
        password,
        kills,
        deaths,
        assits,
        rank,
        games_played,
        days_played,
        status,
    }
}

fn test(){
    println!("{}","test")
}