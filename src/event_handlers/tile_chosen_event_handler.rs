use crate::actions::spawn_piece;
use crate::components::Piece;
use crate::events::{GameOverEvent, TileChosenEvent};
use crate::resources::PlayerTurn;
use crate::states::AppState;
use crate::types::{Coord, PieceSize, Player};
use bevy::prelude::{
    AssetServer, Commands, EventReader, EventWriter, NextState, Query, Res, ResMut,
};
use bevy::utils::HashSet;
use std::collections::HashMap;

pub fn tile_chosen_event_handler(
    mut tile_chosen_event_reader: EventReader<TileChosenEvent>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>,
    mut player_turn: ResMut<PlayerTurn>,
    q_pieces: Query<&Piece>,
) {
    for event in tile_chosen_event_reader.read() {
        let player = event.player().clone();
        let coord = event.coord().clone();
        let piece_size = event.piece_size().clone();
        println!(
            "Player {:?} chose tile {:?} with piece size {:?}",
            player, coord, piece_size
        );
        println!("=============>>>>");

        let validation_result = validate_move(&q_pieces, &coord, &piece_size);
        if !validation_result {
            println!("Invalid move");
            app_state.set(AppState::Playing);
            return;
        }

        let piece = Piece::new(piece_size.clone(), player, coord);
        spawn_piece(&mut commands, &asset_server, piece);

        if validate_board_for_win(&q_pieces, &player, coord.clone(), piece_size.clone()) {
            app_state.set(AppState::GameOver);
            game_over_event_writer.send(GameOverEvent::new(player.clone()));
            return;
        }

        player_turn.switch();
        player_turn.reset_chosen_piece_size();
        app_state.set(AppState::Playing);
    }
}

/// check 3x3 board - horizontal, vertical, diagonal
fn validate_board_for_win(
    q_pieces: &Query<&Piece>,
    player: &Player,
    coord: Coord,
    size: PieceSize,
) -> bool {
    let mut my_pieces: HashMap<(usize, usize), PieceSize> = HashMap::new();
    let mut other_pieces: HashMap<(usize, usize), PieceSize> = HashMap::new();

    my_pieces.insert((coord.x(), coord.y()), size);

    for piece in q_pieces.iter() {
        let coord1 = piece.coord();
        let key = (coord1.x(), coord1.y());

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

fn validate_move(q_pieces: &Query<&Piece>, coord: &Coord, new_value: &PieceSize) -> bool {
    let pieces: Vec<Piece> = q_pieces
        .iter()
        .filter(|piece| piece.coord() == *coord)
        .cloned()
        .collect::<Vec<Piece>>();
    println!("Found pqiece: {:?}", pieces);

    if pieces.is_empty() {
        return true;
    }

    pieces.iter().all(|piece| piece.size() < *new_value)
}
