pub mod constants;
pub mod docs;
pub mod handlers;

use std::{sync::Arc, time::Duration};

use axum::{body::Bytes, http::header};
use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestId,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit, ServiceBuilderExt,
};

use self::constants::HEALTH_CHECK_PATH;

#[derive(Clone)]
pub struct MakeRequestIdUuid7;

impl MakeRequestId for MakeRequestIdUuid7 {
    fn make_request_id<B>(
        &mut self,
        _: &axum::http::Request<B>,
    ) -> Option<tower_http::request_id::RequestId> {
        let request_id = uuid7::new_v7().to_string().parse().unwrap();
        Some(tower_http::request_id::RequestId::new(request_id))
    }
}

#[allow(clippy::unused_async)]
pub async fn application_router() -> axum::Router {
    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();

    let middleware = ServiceBuilder::new()
    .sensitive_request_headers(sensitive_headers.clone())
    .set_x_request_id(MakeRequestIdUuid7)
    .layer(
        TraceLayer::new_for_http()
            .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                tracing::trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk");
            })
            .make_span_with(DefaultMakeSpan::new().include_headers(true))
            .on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros)),
    )
        .sensitive_response_headers(sensitive_headers)
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .compression()
        ;

    let router = axum::Router::new()
        // `GET /health` the `health check` endpoint
        .route(
            HEALTH_CHECK_PATH,
            axum::routing::get(handlers::health_check::healthz),
        )
        .layer(middleware);

    docs::setup(router)
}
