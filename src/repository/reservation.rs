use chrono::Utc;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue,
    ColumnTrait,
    Condition,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
};

use crate::entity::{
    prelude::Reservation,
    reservation::{ActiveModel, Column, Model},
};
 
pub async fn find_reservations_by_event(
    db: &DatabaseConnection,
    event_id: i32,
) -> Vec<Model> {
    Reservation::find()
        .filter(Column::EventId.eq(event_id))
        .all(db)
        .await
        .unwrap()
}
 
pub async fn find_reservations_by_event_and_username_and_phone(
    db: &DatabaseConnection,
    event_id: i32,
    username: &String,
    phone: &String,
) -> Option<Model> {
    Reservation::find()
        .filter(
            Condition::all()
                .add(Column::EventId.eq(event_id))
                .add(Column::Username.eq(username))
                .add(Column::Phone.eq(phone))
        )
        .one(db)
        .await
        .unwrap()
}

pub async fn save_reservation(
    db: &DatabaseConnection,
    event_id: i32,
    username: String,
    phone: String,
) -> Model {
    let new_reservation = ActiveModel {
        id: ActiveValue::NotSet,
        event_id: ActiveValue::Set(event_id),
        username: ActiveValue::Set(username),
        phone: ActiveValue::Set(phone),
        created_dtm: ActiveValue::Set(Utc::now()),
    };
    match new_reservation.insert(db)
        .await
    {
        Ok(model) => model,
        Err(e) => panic!("{}", e),
    }
}
