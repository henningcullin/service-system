use axum::{
    extract::{
        State,
        Json
    },
    http::StatusCode
};
use sqlx::{
    MySqlPool,
    FromRow,
};
use serde_derive::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Facility {
    facility_id: i32,
    name: String
}

pub async fn details() {
    
}

pub async fn index(
    State(pool): State<MySqlPool>
) -> Result<Json<Vec<Facility>>, StatusCode> {
    let facilities = sqlx::query_as::<_, Facility>("SELECT * FROM facility")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    return Ok(Json(facilities));
}