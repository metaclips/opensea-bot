use serenity::http::Http;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::Message;

pub struct Context {
    pub users: Arc<Mutex<HashMap<String, Vec<String>>>>,
    pub stream_rcvr: mpsc::Receiver<Message>,
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
