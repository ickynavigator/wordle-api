use tide::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Game {
    /// the game's creator
    pub creator_id: u64,
}