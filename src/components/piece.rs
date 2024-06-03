use crate::types::{Coord, PieceSize, Player};
use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Piece {
    size: PieceSize,
    player: Player,
    coord: Coord,
}

impl Piece {
    pub fn new(size: PieceSize, player: Player, coord: Coord) -> Self {
        Piece {
            size,
            player,
            coord,
        }
    }

    pub fn size(&self) -> PieceSize {
        self.size.clone()
    }

    pub fn player(&self) -> Player {
        self.player.clone()
    }

    pub fn coord(&self) -> Coord {
        self.coord.clone()
    }
}
