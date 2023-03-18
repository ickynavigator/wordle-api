use tide::{prelude::*, Body, Request, Response};

use crate::{
    middleware::helper::{creator_checks, CreatorCheckResult},
    model::{
        game::{AttemptTypeResults, Game},
        store::Store,
    },
};

/// This endpoint handles the requests made to create a game. It accepts the `creator_id` in it's body
pub async fn create_game(mut req: Request<Store>) -> tide::Result {
    #[derive(Debug, Deserialize, Serialize)]
    struct CreateBody {
        creator_id: String,
    }

    let body: CreateBody = match req.body_json().await {
        Ok(body) => body,
        Err(e) => {
            let mut response = Response::new(400);
            response.set_body(format!("Error parsing body: {}", e));
            return Ok(response);
        }
    };
    let creator_id = body.creator_id;

    let store = req.state();

    match creator_checks(store.get_games(), creator_id.clone()) {
        CreatorCheckResult::NoGameExists => {
            let mut response = Response::new(200);

            let game = Game::new(creator_id.clone());
            store.to_owned().add_game(game);

            if let Some(game) = store.get_games().get(&creator_id) {
                response.set_body(Body::from_json(&game)?);
            } else {
                response.set_status(400);
                response.set_body(format!("Game not created. Please try again"));
            }

            return Ok(response);
        }
        CreatorCheckResult::SameDayGameExists => {
            let mut response = Response::new(400);
            response.set_body(format!("Game already exists"));
            return Ok(response);
        }
    }
}

pub async fn add_attempt(mut req: Request<Store>) -> tide::Result {
    #[derive(Deserialize)]
    pub struct AttemptBody {
        creator_id: String,
        attempt: String,
    }

    let body: AttemptBody = match req.body_json().await {
        Ok(body) => body,
        Err(e) => {
            let mut response = Response::new(400);
            response.set_body(format!("Error parsing body: {}", e));
            return Ok(response);
        }
    };
    let AttemptBody {
        creator_id,
        attempt,
    } = body;

    let store = req.state();
    println!("Store: {:?}", store);

    // first check if an attempt can be made
    let mut game = match store.get_games().get(&creator_id) {
        Some(game) => game.to_owned(),
        None => {
            let mut response = Response::new(400);
            response.set_body(format!("Game not found"));
            return Ok(response);
        }
    };

    let mut response = Response::new(500);

    if game.is_attempt_possible() {
        let result = game.add_attempt(&attempt);

        match result {
            AttemptTypeResults::GameFilled => {
                response.set_status(400);
                response.set_body(format!("Game attempts are filled"));
            }
            AttemptTypeResults::GameWon => {
                response.set_status(200);
                response.set_body(format!("Game has been won"));
            }
            AttemptTypeResults::GameNotWon => {
                response.set_status(200);
                response.set_body(format!("Game has not been won yet"));
            }
        }
    } else {
        response.set_status(400);
        response.set_body(format!("No Attempts are possible"));
    }

    Ok(response)
}

pub async fn fetch_all_games(req: Request<Store>) -> tide::Result {
    let games = { req.state().games.clone() };

    let mut response = Response::new(200);
    response.set_body(Body::from_json(&games)?);

    Ok(response)
}
