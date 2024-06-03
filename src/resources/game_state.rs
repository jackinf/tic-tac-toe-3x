use bevy::prelude::Resource;

#[derive(Resource)]
pub enum GameState {
    Playing,
    Validating,
    GameOver,
}

impl GameState {
    pub fn new() -> Self {
        GameState::Playing
    }

    pub fn set_playing(&mut self) {
        *self = GameState::Playing;
    }

    pub fn validate(&mut self) {
        *self = GameState::Validating;
    }

    pub fn set_game_over(&mut self) {
        *self = GameState::GameOver;
    }
}
