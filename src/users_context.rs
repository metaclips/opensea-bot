use crate::stream::Stream;
use serenity::http::Http;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub struct Context {
    users: Arc<Mutex<HashMap<String, Vec<String>>>>,
    stream_sender: Stream,
    http_link: Arc<Http>,
}

impl Context {
    pub async fn start_opensea_list_stream(&self) {
        todo!()
    }

    pub async fn send_list_to_subscribers(&self) {
        todo!()
    }
}
