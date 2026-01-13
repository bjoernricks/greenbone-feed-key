use crate::{app::App, cli::Cli};

mod app;
mod cli;
mod health;
mod key;
mod openapi;

#[tokio::main]
async fn main() {
    let cli = Cli::default();
    let app = App::new(cli.key_path.into());
    app.serve(&cli.server, cli.port).await.unwrap();
}
