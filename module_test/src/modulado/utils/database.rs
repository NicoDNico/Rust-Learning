
// idk if im correct but i believe using an internal private struct is good patterns?
#[derive(Debug)]
pub struct Player{
    username:String,
    email:String,
    kills:u16,
    deaths:u16,
    assists:u16,
    rank:u8,
    games_played:u16,
    days_played:u16,
    status:bool,
    }

impl Player{
    fn connect(&mut self){
        self.status=true;
    }

}



fn build_player(username:String, password:&str, kills:u16, deaths:u16, assists:u16, rank:u8, games_played:u16, days_played:u16, status:bool)->Player{
    
    // let answer_email:String = auth_user();
    Player{
        username,
        email:String,
        kills,
        deaths,
        assists,
        rank,
        games_played,
        days_played,
        status,
    }
}


#[derive(Debug)]
pub struct CalculatedStats{
    pub username:String,
    pub kda:f32,
    pub rank_per_day:f32,
    pub kills_per_game:f32,
    pub games_per_day:f32,
}
// i want to give this as an answer of the module and not the Player struct.
pub fn build_stats(user: &Player)->CalculatedStats{
    CalculatedStats{
        username:user.username,
        kda:(user.kills as f32+ user.assists as f32) / user.deaths as f32,
        rank_per_day: user.rank as f32 / user.days_played as f32,
        kills_per_game: user.kills as f32/ user.games_played as f32,
        games_per_day:user.games_played as f32/ user.days_played as f32,

    }
}