use crate::{
    app::{GlobalState, create_app},
    cli::Cli,
};

mod app;
mod cli;
mod health;
mod key;
mod openapi;

#[tokio::main]
async fn main() {
    let cli = Cli::default();

    let state = GlobalState::new(cli.key_path.into());
    let app = create_app(state);

    let address = format!("{}:{}", cli.server, cli.port);
    println!("Listening on http://{}", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
