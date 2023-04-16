use actix::{Addr, Message};

use crate::{
    actors::{display::DisplayActor, player::PlayerActor},
    error::ApiError,
};

#[derive(Debug, Message)]
#[rtype(result = "String")]
pub struct CreateGame(pub Addr<DisplayActor>);

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct GameClose(pub String);

#[derive(Debug, Message)]
#[rtype(result = "Result<(), ApiError>")]
pub struct PlayerJoin {
    pub addr: Addr<PlayerActor>,
    pub game_code: String,
    pub player_name: String,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct PlayerLeave {
    pub game_code: String,
    pub name: String,
}
