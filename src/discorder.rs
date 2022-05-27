use crate::{stream::Stream, Result};
use serenity::prelude::{Client, GatewayIntents};

pub async fn start_bot(token: String, ws_stream: Stream) {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(ws_stream)
        .await
        .expect("Error starting discord handler");

    client.start().await.unwrap()
}
