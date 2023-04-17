mod messages;
mod packets;

use crate::actors::{
    server::{self, CreateGame, ServerActor},
    Hb, RunHb,
};
use actix::{
    clock::Instant, fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext,
    ContextFutureSpawner, Handler, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
pub use messages::*;
use tracing::{error, info};

pub struct DisplayActor {
    hb: Instant,
    game_server: Addr<ServerActor>,
    game_code: String,
}

impl DisplayActor {
    pub fn new(game_server: Addr<ServerActor>) -> Self {
        Self {
            hb: Instant::now(),
            game_server,
            game_code: String::new(),
        }
    }
}

impl Hb for DisplayActor {
    fn get_hb(&self) -> Instant {
        self.hb
    }
}

impl Actor for DisplayActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        info!("Socket started");
        self.game_server
            .send(CreateGame(ctx.address()))
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => {
                        act.game_code = res.clone();
                        ctx.text(
                            serde_json::to_string(&packets::SendPackets::Code { code: res })
                                .unwrap(),
                        );
                        ctx.text(
                            serde_json::to_string(&packets::SendPackets::Decks {
                                decks: vec!["base".into()],
                            })
                            .unwrap(),
                        );
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
            .send(server::GameClose(self.game_code.clone()))
            .into_actor(self);

        actix::Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for DisplayActor {
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

impl Handler<PlayerJoin> for DisplayActor {
    type Result = ();

    fn handle(&mut self, msg: PlayerJoin, ctx: &mut Self::Context) {
        ctx.text(
            serde_json::to_string(&packets::SendPackets::PlayerJoin {
                name: msg.0.clone(),
            })
            .unwrap(),
        );
    }
}
