use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Port to listen on
    #[arg(short, long, env = "GREENBONE_FEED_KEY_PORT", default_value_t = 3000)]
    pub port: u16,

    #[arg(short, long, env = "GREENBONE_FEED_KEY_SERVER", default_value_t = String::from("127.0.0.1"))]
    pub server: String,

    /// Path to the key directory
    #[arg(short, long, env = "GREENBONE_FEED_KEY_PATH", default_value_t = String::from("/etc/gvm/greenbone-enterprise-feed-key"))]
    pub key_path: String,
}

impl Default for Cli {
    fn default() -> Cli {
        Cli::parse()
    }
}
