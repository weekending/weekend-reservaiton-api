mod config;
mod core;
mod entity;
mod repository;
mod service;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;

use crate::config::postgres::init_db;
use crate::service::reservation::{create_reservation, get_reservations};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db: DatabaseConnection = init_db().await;

    let app = Router::new()
        .route("/", get(|| async move { "ok" }))
        .route(
            "/reservations", 
            get(get_reservations)
                .post(create_reservation)
        )
        .with_state(db);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}
