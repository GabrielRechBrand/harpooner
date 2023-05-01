use std::error::Error;
use serenity::{http::Http, model::webhook::Webhook};

pub async fn send_message(message: &str) -> Result<(), Box<dyn Error>> {
    let http = Http::new("token");

    let url = std::env::var("HARPOONER_WEBHOOK");
    let webhook = Webhook::from_url(&http, &url.unwrap()).await?;

    webhook
        .execute(&http, true, |w| {
            w.content(message)
        })
        .await?;

    Ok(())
}