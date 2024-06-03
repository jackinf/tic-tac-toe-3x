use crate::types::{Coord, PieceSize, Player};
use bevy::prelude::Event;

#[derive(Event)]
pub struct TileChosenEvent {
    player: Player,
    coord: Coord,
    piece_size: PieceSize,
}

impl TileChosenEvent {
    pub fn new(player: Player, tile_coord: Coord, piece_size: PieceSize) -> Self {
        TileChosenEvent {
            player,
            coord: tile_coord,
            piece_size,
        }
    }

    pub fn player(&self) -> Player {
        self.player.clone()
    }

    pub fn coord(&self) -> Coord {
        self.coord.clone()
    }

    pub fn piece_size(&self) -> PieceSize {
        self.piece_size.clone()
    }
}
