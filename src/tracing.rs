use axum::{
    body::Bytes,
    http::{HeaderMap, Request},
    response::Response,
    Router,
};
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::TraceLayer,
};
use tracing::Span;

use std::time::Duration;

pub fn add_tracing_layer() -> Router {
    return Router::new()
        .layer(
            TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {

                // let matched_path = request
                //     .extensions()
                //     .get::<MatchedPath>()
                //     .map(MatchedPath::as_str);

                tracing::info_span!(
                    "http_request ",
                    // method = ?request.method(),
                    uri = ?request.uri().path(),
                    // user_agent = ?request.headers().get("User-Agent"),
                    // matched_path,
                    // some_other_field = tracing::field::Empty,
                )

            })
            .on_request(|request: &Request<_>, _span: &Span| {
                tracing::info!("request: {} {}", request.method(), request.uri().path())
            })
            .on_response(|response: &Response, latency: Duration, _span: &Span| {
                tracing::info!("response: {} in {:?}", response.status(), latency)
            })
            .on_body_chunk(|chunk: &Bytes, latency: Duration, _span: &Span| {
                tracing::info!("sending {} bytes in {:?}", chunk.len(), latency)
            })
            .on_eos(|_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
                tracing::info!("stream closed after {:?}", stream_duration)
            })
            .on_failure(|error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                tracing::error!("error: {}", error)
            })
        )
}

// Looks like layers will only work for the Router object it's attached to. Can't merge to an existing Router.
// https://docs.rs/axum/latest/axum/routing/struct.Router.html#method.layer