use axum::{Json, extract::{Query, State}};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::core::{code::ApiCode, error::ApiError, response::ApiResponse};
use crate::entity::reservation::Model;
use crate::repository::reservation::{
    find_reservations_by_event,
    find_reservations_by_event_and_username_and_phone,
    save_reservation,
};

#[derive(Deserialize)]
pub struct RequestReservationQuery {
    event_id: i32,
}

#[derive(Deserialize)]
pub struct RequestReservation {
    event_id: i32,
    username: String,
    phone: String,
}

pub async fn get_reservations(
    State(db): State<DatabaseConnection>,
    Query(data): Query<RequestReservationQuery>,
) -> ApiResponse<Vec<Model>> {
    let reservations = find_reservations_by_event(&db, data.event_id)
        .await;
    ApiResponse::with_status(ApiCode::Ok, reservations)
}

pub async fn create_reservation(
    State(db): State<DatabaseConnection>,
    Json(data): Json<RequestReservation>,
) -> Result<ApiResponse<Model>, ApiError> {
    if let Some(_) = find_reservations_by_event_and_username_and_phone(
        &db, data.event_id, &data.username, &data.phone
    ).await
    {
        return Err(ApiError::new(ApiCode::AlreadyReserved));
    }
    let reservation = save_reservation(&db, data.event_id, data.username, data.phone)
        .await;
    Ok(ApiResponse::with_status(ApiCode::Created, reservation))
}
