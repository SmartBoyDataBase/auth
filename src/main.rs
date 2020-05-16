#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use std::env;

use actix_web::{App, HttpServer, web};

use crate::db::PgConnection;
use crate::handler::{login, ping, sign_in};

mod db;
mod service;
mod handler;

lazy_static! {
    pub static ref DB_URL: String = env::var("DB_URL").unwrap();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .data_factory(|| PgConnection::connect(&DB_URL))
            .route("/ping", web::get().to(ping))
            .route("/login", web::post().to(login))
            .route("/sign-in", web::post().to(sign_in))
    }).bind("0.0.0.0:8000")?
        .run()
        .await
}
