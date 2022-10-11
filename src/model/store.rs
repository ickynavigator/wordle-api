use async_std::sync::RwLock;

use crate::model::game::Game;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone, Debug)]
pub struct Store {
    pub games: Arc<RwLock<HashMap<String, Game>>>,
}
