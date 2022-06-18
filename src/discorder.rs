use std::{collections::HashMap, str::FromStr, sync::Arc};

use crate::{
    commands::Command,
    stream::{Stream, Websocket},
    users_context,
};

use async_trait::async_trait;
use futures_util::stream::SplitStream;
use log::{error, warn};
use serenity::{
    http::Http,
    model::prelude::Message as DiscordMessage,
    prelude::{Client, Context, EventHandler, GatewayIntents},
};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::Message as WebsocketMessage;

struct Bot {
    users: Arc<Mutex<HashMap<u64, Vec<String>>>>,
    stream_sender: Stream,
}

pub async fn start_bot(token: String, stream_sender: Stream, stream_rcvr: SplitStream<Websocket>) {
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
    async fn subscribe_user(&self, http: Arc<Http>) {
        todo!()
    }

    async fn unsubscribe_user(&self, http: Arc<Http>) {}

    async fn subscribe_to_listing(&self, http: Arc<Http>, collection: &str) {
        todo!()
    }

    async fn unsubscribe_from_listing(&self, http: Arc<Http>, user_id: u64, listing: &str) {
        let mut user = self.users.lock().await;
        let user_new_collection = {
            user.get_mut(&user_id).and_then(|coll| {
                Some(
                    coll.clone()
                        .into_iter()
                        .filter(|x| listing != x.as_str())
                        .collect::<Vec<String>>(),
                )
            })
        };

        if let Some(e) = user_new_collection {
            user.insert(user_id, e);
        }
    }
}

#[async_trait]
impl EventHandler for Bot {
    /// Load messages from discord and parse it to a useable command
    async fn message(&self, ctx: Context, msg: DiscordMessage) {
        let user_id = msg.author.id.0;
        let message = match Command::from_str(msg.content.as_str()) {
            Ok(e) => e,
            Err(e) => {
                warn!("Error converting discord message to a supported command: {e}");
                if let Err(e) = msg.channel_id.say(ctx.http, "Message unrecognized").await {
                    error!("Error sending message to discord user: {:?}", e);
                }

                return;
            }
        };

        match message {
            Command::Subscribe => {
                self.subscribe_user(ctx.http).await;
            }
            Command::Unsubscribe => {
                self.unsubscribe_user(ctx.http).await;
            }
            Command::ListCollection(e) => {
                self.subscribe_to_listing(ctx.http, &e).await;
            }
            Command::UnlistCollection(e) => {
                self.unsubscribe_from_listing(ctx.http, user_id, &e).await;
            }
        };
    }
}
