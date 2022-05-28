use crate::Network;
use crate::Result;
use log::debug;

use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message as WSMessage};

pub struct Stream {
    sender: mpsc::Sender<WSMessage>,
}

impl Stream {
    pub async fn new(network: Network, token: String) -> (Self, mpsc::Receiver<WSMessage>) {
        let (ws, resp) = connect_async(network.to_string())
            .await
            .expect("Error connecting websocket");
        debug!("Websocket handshake completed with {:?}", resp.body());

        todo!()
    }

    pub async fn send_stream(&self, message: WSMessage) -> Result<()> {
        todo!()
    }
}
