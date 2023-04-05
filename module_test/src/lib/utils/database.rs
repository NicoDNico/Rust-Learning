

#[derive(Debug,Clone)]
pub struct Player{
    pub username:String,
    pub password:String,
    pub kills:u16,
    pub deaths:u16,
    pub assists:u16,
    pub rank:u8,
    pub games_played:u16,
    pub days_played:u16,
    pub status:bool,
    }

impl Player{
    fn connect(&mut self){
        self.status=true;
    }

    fn get_all(self)->Player{
        self.clone()
    }
}
#[derive(Debug)]
pub struct CalculatedStats{
    username:String,
    kda:f32,
    rank_per_day:f32,
    kills_per_game:f32,
    games_per_day:f32,
}

fn build_stats(user: Player)->CalculatedStats{
    CalculatedStats{
        username:user.username,
        kda:(user.kills as f32+ user.assists as f32) / user.deaths as f32,
        rank_per_day: user.rank as f32 / user.days_played as f32,
        kills_per_game: user.kills as f32/ user.games_played as f32,
        games_per_day:user.games_played as f32/ user.days_played as f32,

    }
}