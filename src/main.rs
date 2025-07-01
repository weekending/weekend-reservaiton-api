mod config;
mod core;
mod entity;
mod repository;
mod service;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use tracing::info;

use crate::config::{
    logging::{layer::get_trace_layer, registry::init_logging},
    postgres::init_db,
};
use crate::service::reservation::{create_reservation, get_reservations};

#[tokio::main]
async fn main() {
    dotenv().ok();

    init_logging().await;

    let db: DatabaseConnection = init_db().await;
    info!("Connect Database!");

    let app = Router::new()
        .route("/", get(|| async move { "ok" }))
        .route(
            "/reservations", 
            get(get_reservations)
                .post(create_reservation)
        )
        .layer(get_trace_layer())
        .with_state(db);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}
