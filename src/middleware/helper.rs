use crate::{lib::lib, model::game::Game};
use std::{collections::HashMap, fmt};

pub enum CreatorCheckResult {
    SameDayGameExists,
    NoGameExists,
}

impl fmt::Display for CreatorCheckResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NoGameExists => "No game exists",
                Self::SameDayGameExists => "A Game was created today already",
            }
        )
    }
}

pub fn creator_checks(game: HashMap<String, Game>, creator_id: String) -> CreatorCheckResult {
    if game.contains_key(&creator_id) {
        let game = game.get(&creator_id).unwrap();

        let is_same_day = lib::check_if_same_day(game.start_time);
        if is_same_day {
            return CreatorCheckResult::SameDayGameExists;
        }
    }

    return CreatorCheckResult::NoGameExists;
}
