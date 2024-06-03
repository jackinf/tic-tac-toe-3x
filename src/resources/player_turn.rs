use crate::types::{PieceSize, Player};
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct PlayerTurn {
    player: Player,
    chosen_piece_size: Option<PieceSize>,
}

impl PlayerTurn {
    pub fn new() -> Self {
        PlayerTurn {
            player: Player::One,
            chosen_piece_size: None,
        }
    }

    pub fn player(&self) -> Player {
        self.player.clone()
    }

    pub fn switch(&mut self) {
        self.player = match self.player {
            Player::One => Player::Two,
            Player::Two => Player::One,
        };
    }

    pub fn set_chosen_piece_size(&mut self, piece_size: PieceSize) {
        self.chosen_piece_size = Some(piece_size);
    }

    pub fn chosen_piece_size(&self) -> Option<PieceSize> {
        self.chosen_piece_size.clone()
    }

    pub fn reset_chosen_piece_size(&mut self) {
        self.chosen_piece_size = None;
    }
}
