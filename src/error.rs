use std::fmt::Display;

pub enum Error {
    WS(tokio_tungstenite::tungstenite::Error),
    ParseCommand(String),
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::WS(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::WS(ref e) => write!(f, "Tungstenite error: {:?}", e),
            Self::ParseCommand(ref e) => write!(f, "Error parsing command message: {:?}", e),
        }
    }
}
