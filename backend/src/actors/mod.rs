use std::{fmt::DebugStruct, time::Duration};

use actix::{clock::Instant, Actor, ActorContext};
use actix_web_actors::ws;
use tracing::warn;

pub mod display;
pub mod player;
pub mod server;

const HB_INTERVAL: Duration = Duration::from_secs(5);
const HB_TIMEOUT: Duration = std::time::Duration::from_secs(15);

trait Hb {
    fn get_hb(&self) -> Instant;
}

trait RunHb
where
    Self: Actor,
{
    fn hb(&self, ctx: &mut Self::Context);
}

impl<T> RunHb for T
where
    T: Hb + Actor<Context = ws::WebsocketContext<T>>,
{
    fn hb(&self, ctx: &mut Self::Context) {
        // check client heartbeats
        if Instant::now().duration_since(self.get_hb()) > HB_TIMEOUT {
            // notify chat server
            // act.addr.do_send(server::Disconnect { id: act.id });
            warn!("HB Failed");

            // stop actor
            ctx.stop();

            // don't try to send a ping
            return;
        }

        ctx.ping(b"");
    }
}
