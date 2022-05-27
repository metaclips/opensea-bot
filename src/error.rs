pub enum Error {
    WS(tokio_tungstenite::tungstenite::Error),
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::WS(e)
    }
}
