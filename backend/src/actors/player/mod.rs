mod messages;
mod packets;

use self::packets::SendPackets;
use crate::{
    actors::server::PlayerJoin,
    actors::{
        server::{self, ServerActor},
        Hb, RunHb,
    },
    error::ApiError,
};
use actix::{
    clock::Instant, fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext,
    ContextFutureSpawner, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
pub use messages::*;
use tracing::{error, info};

pub struct PlayerActor {
    game_server: Addr<ServerActor>,
    game_code: String,
    name: String,
    hb: Instant,
}

impl PlayerActor {
    pub fn new(game_server: Addr<ServerActor>, game_code: String, name: String) -> Self {
        Self {
            game_server,
            game_code,
            name,
            hb: Instant::now(),
        }
    }
}

impl Hb for PlayerActor {
    fn get_hb(&self) -> Instant {
        self.hb
    }
}

impl Actor for PlayerActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        // info!("Socket started");
        self.game_server
            .send(PlayerJoin {
                addr: ctx.address(),
                game_code: self.game_code.clone(),
                player_name: self.name.clone(),
            })
            .into_actor(self)
            .then(|res, slf, ctx| {
                match res {
                    Ok(Ok(())) => {
                        ctx.text(
                            serde_json::to_string(&SendPackets::JoinSuccess {
                                game_code: slf.game_code.clone(),
                                name: slf.name.clone(),
                            })
                            .unwrap(),
                        );
                    }
                    Ok(Err(ApiError::NameTaken)) => {
                        ctx.text(serde_json::to_string(&SendPackets::NameTaken).unwrap());
                        ctx.stop();
                    }
                    Ok(Err(ApiError::NoGameCode)) => {
                        ctx.text(serde_json::to_string(&SendPackets::NoGameCode).unwrap());
                        ctx.stop();
                    }
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        self.game_server
            .send(server::PlayerLeave {
                game_code: self.game_code.clone(),
                name: self.name.clone(),
            })
            .into_actor(self);

        actix::Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PlayerActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(ex) => {
                error!("{ex}");
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Text(msg) => {
                println!("{msg}");
            }
            // ws::Message::Binary(_) => todo!(),
            // ws::Message::Continuation(_) => todo!(),
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => self.hb = Instant::now(),
            ws::Message::Close(_) => {
                info!("Socket closed");
                ctx.stop();
            } // ws::Message::Nop => todo!(),
            _ => info!("Unknown message"),
        }
    }
}
