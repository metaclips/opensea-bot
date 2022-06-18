use futures_util::stream::SplitStream;
use serenity::http::Http;
use std::{collections::HashMap, net::TcpStream, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::stream::Websocket;

pub struct Context {
    pub users: Arc<Mutex<HashMap<u64, Vec<String>>>>,
    pub stream_rcvr: SplitStream<Websocket>,
    pub http_link: Arc<Http>,
}

impl Context {
    pub async fn start_opensea_list_stream(self) {
        todo!()
    }

    async fn send_list_to_subscribers(&self) {
        todo!()
    }
}
