use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub username: String,
    pub uid: Uuid,
    pub rating: u64,
    pub played: i64,
    pub win: i64,
    pub loss: i64,
    pub draw: i64,
}

use cfg_if::cfg_if;
cfg_if! { if #[cfg(feature = "ssr")] {
use db_lib::{
    models::{rating::Rating, user::User},
    DbPool,
};
use leptos::ServerFnError;
impl UserResponse {
    pub async fn from_uuid(id: &Uuid, pool: &DbPool) -> Result<Self, ServerFnError> {
        let user = User::find_by_uuid(id, pool).await?;
        let rating = Rating::for_uuid(id, pool).await?;

        Ok(Self {
            username: user.username,
            uid: user.id,
            rating: rating.rating.floor() as u64,
            played: rating.played,
            win: rating.won,
            loss: rating.lost,
            draw: rating.draw,
        })
    }

    pub async fn from_username(username: &str, pool: &DbPool) -> Result<Self, ServerFnError> {
        let user = User::find_by_username(username, pool).await?;
        let rating = Rating::for_uuid(&user.id, pool).await?;

        Ok(Self {
            username: user.username,
            uid: user.id,
            rating: rating.rating.floor() as u64,
            played: rating.played,
            win: rating.won,
            loss: rating.lost,
            draw: rating.draw,
        })
    }
}
}}