#![allow(warnings)]

use actix::{Actor, Addr};
use actix_web::{
    get,
    web::{self, Data, Path, Query},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use actors::{display::DisplayActor, server::ServerActor};
use serde::Deserialize;
use std::io;
use tracing::{debug, info};
use tracing_subscriber::filter::LevelFilter;

use crate::{
    actors::{player::PlayerActor, server::PlayerJoin},
    error::ApiError,
};

mod actors;
mod error;

#[actix_web::main]
async fn main() -> io::Result<()> {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt().init();

    debug!("Running debug build");
    info!("Starting on http://0.0.0.0:8080");

    let server = ServerActor::new().start();

    HttpServer::new(move || {
        App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(game_route)
            .service(display_route)
            .app_data(Data::new(server.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[derive(Debug, Clone, Deserialize)]
struct GameQueryParams {
    name: String,
}

#[get("/display")]
async fn display_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ServerActor>>,
) -> Result<HttpResponse, Error> {
    info!("Hit game route");
    ws::start(DisplayActor::new(srv.get_ref().clone()), &req, stream)
}

#[get("/game/{game}")]
async fn game_route(
    game_code: Path<(String)>,
    params: Query<GameQueryParams>,
    srv: web::Data<Addr<ServerActor>>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    info!("Hit game route");

    ws::start(
        PlayerActor::new(
            srv.get_ref().clone(),
            game_code.into_inner(),
            params.name.clone(),
        ),
        &req,
        stream,
    )
}
