use std::{path::PathBuf, sync::Arc};

use axum::{Router, http::StatusCode, response::IntoResponse};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, decompression::RequestDecompressionLayer};

#[derive(Clone)]
pub struct GlobalState {
    pub key_path: PathBuf,
}

impl GlobalState {
    pub fn new(key_path: PathBuf) -> Self {
        Self { key_path }
    }
}

pub type AppState = Arc<GlobalState>;
pub type AppRouter = Router<AppState>;

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

pub fn create_app(state: GlobalState) -> Router {
    Router::new()
        .nest("/key", crate::key::routes())
        .nest("/health", crate::health::routes())
        .merge(crate::openapi::routes())
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        )
        .with_state(Arc::new(state))
        .fallback(handler_404)
}
