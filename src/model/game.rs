use chrono::Utc;
use sqlx::FromRow;
use std::fmt;
use tide::prelude::*;
use uuid::Uuid;

use crate::lib::lib;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AttemptTypeResults {
    GameFilled,
    GameWon,
    GameNotWon,
}

impl fmt::Display for AttemptTypeResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::GameFilled => "Game attempts are filled",
                Self::GameWon => "Game has been won",
                Self::GameNotWon => "Game has not been won yet",
            }
        )
    }
}

/// The game model for the wordle session.
#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Game {
    /// Game ID
    pub id: String,
    /// Game's creator
    pub creator_id: String,
    /// Correct Answer
    pub answer: String,
    /// Tries the user made
    pub attempts: Vec<String>,
    /// Time the game started
    pub start_time: i64,
}

pub const GAME_MAX_ATTEMPT: usize = 6;
impl Game {
    /// Create a new game session with a given creator ID.
    ///
    /// Uses the timestamp to generate a start time
    pub fn new(creator_id: &String) -> Game {
        let start_time = Utc::now().timestamp();
        let id = Uuid::new_v4().to_string();
        let answer = ("ANSWER").to_string();

        Game {
            id,
            creator_id: creator_id.clone(),
            answer,
            attempts: vec![],
            start_time,
        }
    }

    pub fn add_attempt(&mut self, attempt: &str) -> AttemptTypeResults {
        if self.attempts.len() >= GAME_MAX_ATTEMPT {
            return AttemptTypeResults::GameFilled;
        }

        self.attempts.push(attempt.to_string());

        if attempt.to_lowercase() == self.answer.to_lowercase() {
            return AttemptTypeResults::GameWon;
        } else {
            return AttemptTypeResults::GameNotWon;
        }
    }

    pub fn is_attempt_possible(&self) -> bool {
        if self.attempts.len() >= GAME_MAX_ATTEMPT {
            println!("Game attempts are filled");
            return false;
        }

        if self.attempts.contains(&self.answer) {
            println!("Game has been won");
            return false;
        }

        if lib::check_if_same_day(self.start_time.clone()) {
            return true;
        }

        println!("Game has not been won yet");
        true
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Game {{ id: {}, creator_id: {}, answer: {}, attempts: {:?}, start_time: {} }}",
            self.id, self.creator_id, self.answer, self.attempts, self.start_time
        )
    }
}
