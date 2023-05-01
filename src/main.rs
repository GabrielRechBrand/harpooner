#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use mongodb::bson::doc;
use rocket::Config;
use rocket::config::Environment;
use tokio;
use tokio::net::windows::named_pipe::PipeEnd::Client;
use database::connection;
use rand::seq::SliceRandom;

mod database {
    pub mod connection;
}

mod utils {
    pub mod app;
    pub mod date;
    pub mod webhook;
}


#[tokio::main]
async fn main() {
    let database = connection::create_client().await.unwrap();
    let harpooner_message = utils::app::HARPOONER_MESSAGES.choose(&mut rand::thread_rng()).unwrap();

    utils::webhook::send_message(harpooner_message).await.unwrap();

    database.collection("log").insert_one(doc! { "message": harpooner_message, "date": utils::date::get_current_datetime().to_string() }, None).await.unwrap();

    let config = Config::build(Environment::Development)
        .port(8080)
        .finalize();

    rocket::custom(config.unwrap())
        .mount("/", routes![])
        .launch();
}