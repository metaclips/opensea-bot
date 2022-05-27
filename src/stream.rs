use crate::Network;
use crate::Result;
use async_trait::async_trait;
use log::debug;
use serenity::{
    model::prelude::Message,
    prelude::{Context, EventHandler},
};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message as WSMessage};

pub struct Stream {
    sender: mpsc::Sender<WSMessage>,
}

impl Stream {
    pub async fn new(network: Network, token: String) -> Self {
        let (ws, resp) = connect_async(network.to_string())
            .await
            .expect("Error connecting websocket");
        debug!("Websocket handshake completed with {:?}", resp.body());

        todo!()
    }

    pub async fn send_stream(&self, message: Message) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl EventHandler for Stream {
    async fn message(&self, ctx: Context, msg: Message) {
        todo!()
    }
}
