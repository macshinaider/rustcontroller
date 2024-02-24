use axum::{handler::get, http::StatusCode, Router, Json, response::IntoResponse};
use serde::Deserialize;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[derive(Deserialize)]
struct Params {
    message: String,
}

async fn endpoint(params: Json<Params>) -> impl IntoResponse {
    let params: Params = params.0;
    format!("Você enviou a mensagem: {}", params.message).into_response()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/:message", get(endpoint));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
