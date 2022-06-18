use clap::Parser;

#[derive(Parser)]
#[clap(author = "metaclips", version = "0.1.0", about = "opensea bot")]
pub struct App {
    /// Opensea API key
    #[clap(long, default_value = "")]
    pub opensea_api_key: String,
    /// Discord API key
    #[clap(long)]
    pub discord_key: String,
    /// Indicates if the bot should run on testnet
    #[clap(long)]
    pub is_testnet: bool,
}
