use std::{collections::HashMap, sync::Arc};

use crate::{stream::Stream, users_context};
use async_trait::async_trait;
use serenity::{
    model::prelude::Message as DiscordMessage,
    prelude::{Client, Context, EventHandler, GatewayIntents},
};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::Message as WebsocketMessage;

struct Bot {
    users: Arc<Mutex<HashMap<String, Vec<String>>>>,
    stream_sender: Stream,
}

pub async fn start_bot(
    token: String,
    stream_sender: Stream,
    stream_rcvr: mpsc::Receiver<WebsocketMessage>,
) {
    let users = Arc::new(Mutex::new(HashMap::new()));

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let bot = Bot {
        users: users.clone(),
        stream_sender,
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(bot)
        .await
        .expect("Error starting discord handler");

    let context = users_context::Context {
        users: users.clone(),
        http_link: client.cache_and_http.http.clone(),
        stream_rcvr,
    };

    tokio::spawn(async move { context.start_opensea_list_stream().await });
    client.start().await.unwrap()
}

impl Bot {
    async fn subscribe_user(&self) {
        todo!()
    }

    async fn subscribe_to_listing(&self) {
        todo!()
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: DiscordMessage) {
        todo!()
    }
}
