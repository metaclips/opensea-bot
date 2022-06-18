mod arg;
mod commands;
mod discorder;
mod error;
mod stream;
mod users_context;
use clap::Parser;

pub type Result<T> = std::result::Result<T, error::Error>;

pub enum Network {
    Mainnet,
    Testnet,
}

impl ToString for Network {
    fn to_string(&self) -> String {
        match *self {
            Network::Mainnet => "wss://stream.openseabeta.com/socket".to_string(),
            Network::Testnet => "wss://testnets-stream.openseabeta.com/socket".to_string(),
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = arg::App::parse();
    let (stream, recvr) = if cli.is_testnet {
        stream::Stream::new(Network::Testnet, cli.opensea_api_key).await
    } else {
        stream::Stream::new(Network::Mainnet, cli.opensea_api_key).await
    };

    discorder::start_bot(cli.discord_key, stream, recvr).await
}
