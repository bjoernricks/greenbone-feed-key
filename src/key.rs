use axum::{
    Router,
    body::Body,
    extract::{Request, State},
    http::{StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use tokio::io::BufWriter;
use tokio_stream::StreamExt;
use tokio_util::io::{ReaderStream, StreamReader};
use utoipa::OpenApi;

use crate::app::{AppRouter, AppState};

const KEY_TAG: &str = "Key";

#[derive(OpenApi)]
#[openapi(
    info(description = "Key management endpoint", title = "Key API"),
    tags((name = KEY_TAG, description = "Key management operations")),
    paths(download_key, upload_key, delete_key)
)]
pub struct KeyApi;

#[utoipa::path(
  get,
  path = "",
  responses(
    (status = 200, description = "Key download OK", body = String, content_type = "application/x-pem-file"),
    (status = 404, description = "Key not available", body = String),
  ),
  tag = KEY_TAG,
)]
async fn download_key(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let file = match tokio::fs::File::open(&state.key_path).await {
        Ok(file) => file,
        Err(_) => {
            return Err((StatusCode::NOT_FOUND, "Key not available"));
        }
    };
    let filename = match state.key_path.file_name() {
        Some(name) => name,
        None => {
            return Err((StatusCode::BAD_REQUEST, "File name couldn't be determined"));
        }
    };
    let content_disposition = format!("attachment; filename=\"{}\"", filename.to_string_lossy());
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let headers = [
        (header::CONTENT_TYPE, "application/x-pem-file".to_string()),
        (header::CONTENT_DISPOSITION, content_disposition),
        (header::CONTENT_LENGTH, "".to_string()),
        (header::TRANSFER_ENCODING, "chunked".to_string()),
    ];
    Ok((headers, body))
}

#[utoipa::path(
  post,
  path = "",
  responses(
    (status = 200, description = "Key upload OK", body = String),
    (status = 500, description = "Key upload failed", body = String),
  ),
  tag = KEY_TAG,
  request_body(content_type = "application/x-pem-file", description = "The key file to upload")
)]
async fn upload_key(State(state): State<AppState>, request: Request) -> impl IntoResponse {
    let file = match tokio::fs::File::create(&state.key_path).await {
        Ok(file) => file,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Key File {} not found: {}", state.key_path.display(), err),
            ));
        }
    };
    let stream = request.into_body().into_data_stream();
    let stream = stream
        .map(|result| result.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err)));
    let mut reader = StreamReader::new(stream);
    let mut writer = BufWriter::new(file);
    match tokio::io::copy(&mut reader, &mut writer).await {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Failed to write key file {}: {}",
                    state.key_path.display(),
                    err
                ),
            ));
        }
    }
}

#[utoipa::path(
  delete,
  path = "",
  responses(
    (status = 200, description = "Key deleted successfully", body = String),
    (status = 500, description = "Key deletion failed", body = String),
  ),
  tag = KEY_TAG,
)]
async fn delete_key(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match tokio::fs::remove_file(&state.key_path).await {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Failed to delete key file {}: {}",
                    state.key_path.display(),
                    err
                ),
            ));
        }
    }
}

pub fn routes() -> AppRouter {
    Router::new().route("/", get(download_key).post(upload_key).delete(delete_key))
}
