use chrono::Utc;
use tide::prelude::*;
use uuid::Uuid;

/// The game model for the wordle session.
#[derive(Debug, Deserialize)]
pub struct Game {
    /// Game ID
    pub id: String,
    /// Game's creator
    pub creator_id: String,
    /// Tries the user made
    pub attempts: Vec<String>,
    /// Time the game started
    pub start_time: i64,
}

impl Game {
    /// Create a new game session with a given creator ID.
    /// Uses the timestamp to generate a start time
    pub fn new_game(creator_id: String) -> Game {
        let timestamp = Utc::now().timestamp();

        Game {
            id: Uuid::new_v4().to_string(),
            creator_id,
            attempts: vec![],
            start_time: timestamp,
        }
    }

    /// Push a new attempt on the game session
    pub fn add_attempt(&mut self, attempt: String) {
        self.attempts.push(attempt);
    }
}
