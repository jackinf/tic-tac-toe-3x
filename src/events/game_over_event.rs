use crate::types::Player;
use bevy::prelude::Event;

#[derive(Event)]
pub struct GameOverEvent {
    winner: Player,
}

impl GameOverEvent {
    pub fn new(winner: Player) -> Self {
        GameOverEvent { winner }
    }

    pub fn winner(&self) -> Player {
        self.winner
    }
}
