mod messages;

use crate::{
    actors::{display::DisplayActor, player::PlayerActor},
    error::ApiError,
};
use actix::{Actor, Addr, Handler};
pub use messages::*;
use rand::{thread_rng, Rng};
use std::collections::{hash_map::Entry, HashMap};

pub struct ServerActor {
    games: HashMap<String, Addr<DisplayActor>>,
    players: HashMap<String, HashMap<String, Addr<PlayerActor>>>,
}

impl ServerActor {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
            players: HashMap::new(),
        }
    }
}

impl Actor for ServerActor {
    type Context = actix::Context<Self>;
}

impl Handler<CreateGame> for ServerActor {
    type Result = String;

    fn handle(&mut self, CreateGame(addr): CreateGame, _ctx: &mut Self::Context) -> Self::Result {
        let mut rng = thread_rng();

        let mut key;

        while {
            key = (0..6)
                .map(|_| rng.sample(rand::distributions::Alphanumeric))
                .map(char::from)
                .collect::<String>()
                .to_ascii_uppercase();
            self.games.contains_key(&key)
        } {}

        self.games.insert(key.clone(), addr);
        self.players.insert(key.clone(), HashMap::new());

        key
    }
}

impl Handler<GameClose> for ServerActor {
    type Result = ();

    fn handle(&mut self, GameClose(key): GameClose, _ctx: &mut Self::Context) -> Self::Result {
        self.games.remove(&key);
    }
}

impl Handler<PlayerJoin> for ServerActor {
    type Result = Result<(), ApiError>;

    fn handle(
        &mut self,
        PlayerJoin {
            addr,
            game_code,
            player_name,
        }: PlayerJoin,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        self.players
            .get_mut(&game_code)
            .ok_or(ApiError::NoGameCode)
            .and_then(|players| {
                if let Entry::Vacant(entry) = players.entry(player_name.clone()) {
                    entry.insert(addr);
                    Ok(())
                } else {
                    Err(ApiError::NameTaken)
                }
            })
    }
}

impl Handler<PlayerLeave> for ServerActor {
    type Result = ();

    fn handle(
        &mut self,
        PlayerLeave { game_code, name }: PlayerLeave,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        if let Some(players) = self.players.get_mut(&game_code) {
            players.retain(|player_name, _| &name == player_name);
        }
    }
}
