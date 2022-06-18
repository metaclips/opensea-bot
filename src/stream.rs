use crate::Network;
use crate::Result;
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use log::debug;

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message as WSMessage,
    tungstenite::{handshake::client::Request, Message},
};

/// TLS or TCP Websocket connection connection.
pub type Websocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct Stream {
    sender: SplitSink<Websocket, Message>,
}

impl Stream {
    pub async fn new(network: Network, token: String) -> (Self, SplitStream<Websocket>) {
        let request = Request::get(network.to_string())
            .extension(token)
            .body(())
            .expect("Error create http request");

        let (ws, resp) = connect_async("wss://testnets-stream.openseabeta.com")
            .await
            .expect("Error connecting websocket");

        debug!("Websocket handshake completed with {:?}", resp.body());

        let (sender, ws_recvr) = ws.split();

        (Self { sender }, ws_recvr)
    }

    pub async fn send_stream(&mut self, message: WSMessage) -> Result<()> {
        self.sender.send(message).await?;

        Ok(())
    }
}
