use crate::components::Piece;
use crate::types::{PieceSize, Player};
use bevy::utils::HashSet;
use std::collections::HashMap;

/// check 3x3 board - horizontal, vertical, diagonal
pub fn validate_board_for_win(q_pieces: &Vec<Piece>, player: &Player) -> bool {
    let mut my_pieces: HashMap<(usize, usize), PieceSize> = HashMap::new();
    let mut other_pieces: HashMap<(usize, usize), PieceSize> = HashMap::new();

    for piece in q_pieces.iter() {
        let coord = piece.coord();
        let key = (coord.x(), coord.y());

        if piece.player() == *player {
            let entry = my_pieces.entry(key).or_insert(piece.size());
            if piece.size() > *entry {
                *entry = piece.size();
            }
        } else {
            let entry = other_pieces.entry(key).or_insert(piece.size());
            if piece.size() > *entry {
                *entry = piece.size();
            }
        }
    }

    let mut xys: HashSet<(usize, usize)> = HashSet::new();

    for (coord, size) in my_pieces.iter() {
        if let Some(other_piece_size) = other_pieces.get(coord) {
            if size > other_piece_size {
                xys.insert(*coord);
            }
        } else {
            xys.insert(*coord);
        }
    }

    let x0 = xys.contains(&(0, 0)) && xys.contains(&(0, 1)) && xys.contains(&(0, 2));
    let x1 = xys.contains(&(1, 0)) && xys.contains(&(1, 1)) && xys.contains(&(1, 2));
    let x2 = xys.contains(&(2, 0)) && xys.contains(&(2, 1)) && xys.contains(&(2, 2));

    let y0 = xys.contains(&(0, 0)) && xys.contains(&(1, 0)) && xys.contains(&(2, 0));
    let y1 = xys.contains(&(0, 1)) && xys.contains(&(1, 1)) && xys.contains(&(2, 1));
    let y2 = xys.contains(&(0, 2)) && xys.contains(&(1, 2)) && xys.contains(&(2, 2));

    let d0 = xys.contains(&(0, 0)) && xys.contains(&(1, 1)) && xys.contains(&(2, 2));
    let d1 = xys.contains(&(0, 2)) && xys.contains(&(1, 1)) && xys.contains(&(2, 0));

    println!(
        "Winning: x0: {}, x1: {}, x2: {}, y0: {}, y1: {}, y2: {}, d0: {}, d1: {}",
        x0, x1, x2, y0, y1, y2, d0, d1
    );

    x0 || x1 || x2 || y0 || y1 || y2 || d0 || d1
}
