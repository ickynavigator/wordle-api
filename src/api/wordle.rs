use crate::model::game::Game;
use tide::Request;

pub async fn test_game(mut req: Request<()>) -> tide::Result {
    let Game { creator_id } = req.body_json().await?;
    Ok(format!("Game created by user {}", creator_id).into())
}