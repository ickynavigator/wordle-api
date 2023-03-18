use crate::model::game::Game;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Store {
    pub games: HashMap<String, Game>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }
    pub fn get_games(&self) -> HashMap<String, Game> {
        self.games.clone()
    }
    pub fn add_game(&mut self, game: Game) {
        let mut games = self.games.clone();
        games.insert(game.creator_id.clone(), game);

        self.games = games;
    }
}
