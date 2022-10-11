use tide::{prelude::*, Body, Request, Response};

use crate::lib::lib;
use crate::model::{game::Game, store::Store};

/// This endpoint handles the requests made to create a game. It accepts the `creator_id` in it's body
pub async fn create_game(mut req: Request<Store>) -> tide::Result {
    #[derive(Debug, Deserialize)]
    struct CreateBody {
        creator_id: String,
    }

    let body: CreateBody = req.body_json().await?;
    let creator_id = body.creator_id;

    // check if creator id is empty
    if creator_id.is_empty() {
        let mut response = Response::new(400);
        response.set_body(format!("creator_id cannot be empty"));

        return Ok(response);
    }

    let mut store = req.state().games.write().await;

    // if game has already been created by that user that day
    if store.contains_key(&creator_id) {
        let game = store.get(&creator_id).unwrap();

        let is_same_day = lib::check_if_same_day(game.start_time);
        if is_same_day {
            let mut response = Response::new(200);
            response.set_body(Body::from_json(&game)?);

            return Ok(response);
        }

        let mut response = Response::new(400);
        response.set_body(format!(
            "game already exists for creator_id: {}",
            creator_id
        ));

        return Ok(response);
    }

    let game = Game::new_game(creator_id.clone());
    store.insert(String::from(&creator_id), game.clone());

    let mut response = Response::new(200);

    if let Some(game) = store.get(&creator_id) {
        response.set_body(Body::from_json(&game)?);
    } else {
        response.set_status(400);
        response.set_body(format!("Game not created. Please try again"));
    }

    return Ok(response);
}
