mod config;
mod core;
mod entity;
mod repository;
mod service;

use std::time::Duration;

use axum::{Router, http::{Request, Response}, routing::get};
use dotenvy::dotenv;
use sea_orm::{DatabaseConnection, prelude::Uuid};
use tower_http::trace::TraceLayer;
use tracing::{Span, info, info_span};
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::config::postgres::init_db;
use crate::service::reservation::{create_reservation, get_reservations};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let file_appender = rolling::never("logs", "api.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    Box::leak(Box::new(_guard));

    tracing_subscriber::registry()
        .with(fmt::layer()
            .with_writer(std::io::stdout)
            .with_ansi(false)
        )
        .with(fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false)
        )
        .with(EnvFilter::from_default_env())
        .init();

    let db: DatabaseConnection = init_db().await;
    info!("Connect Database!");

    let app = Router::new()
        .route("/", get(|| async move { "ok" }))
        .route(
            "/reservations", 
            get(get_reservations)
                .post(create_reservation)
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<_>| {
                    let trace_id = Uuid::new_v4().to_string();
                    info_span!("trace", %trace_id)
                })
                .on_request(|request: &Request<_>, _span: &Span| {
                    info!("Request - {} {}", request.method(), request.uri());
                })
                .on_response(|response: &Response<_>, _latency: Duration, _span: &Span| {
                    info!("Response - {}", response.status());
                })
        )
        .with_state(db);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}
