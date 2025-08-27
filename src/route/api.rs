use crate::handler;
use axum::{Router, routing::get};
use axum_kit::middleware::{compression, request_id, trace};
use tower::ServiceBuilder;
pub fn init() -> Router {
    Router::new()
        .route("/", get(handler::chat::index))
        .route("/ws", get(handler::chat::websocket_handler))
        .layer(
            ServiceBuilder::new()
                .layer(compression::compression())
                .layer(request_id::set_request_id())
                .layer(request_id::propagate_request_id())
                .layer(trace::trace()),
        )
}
