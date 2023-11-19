use crate::functions::{users::user_response::UserResponse, games::game_response::GameStateResponse};
use leptos::*;
use uuid::Uuid;

#[server]
pub async fn get_user_by_uuid(uuid: Uuid) -> Result<UserResponse, ServerFnError> {
    use crate::functions::db::pool;
    let pool = pool()?;
    UserResponse::from_uuid(&uuid, &pool).await
}

#[server]
pub async fn get_user_by_username(username: String) -> Result<UserResponse, ServerFnError> {
    use crate::functions::db::pool;
    let pool = pool()?;
    UserResponse::from_username(&username, &pool).await
}

#[server]
pub async fn get_user_games(username: String) -> Result<Vec<GameStateResponse>, ServerFnError> {
    use crate::functions::db::pool;
    use db_lib::models::{user::User, game::Game};
    let pool = pool()?;
    let games:Vec<Game> = User::find_by_username(&username, &pool).await?.get_games(&pool).await?;
    let mut results: Vec<GameStateResponse>= Vec::new();
    for game in games.iter() {
        if let Ok(game_response) = GameStateResponse::new_from_db(&game, &pool).await {
            results.push(game_response);
        }
    }
    Ok(results)
}