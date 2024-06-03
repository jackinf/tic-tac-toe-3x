use crate::actions::spawn_piece;
use crate::components::Piece;
use crate::events::{GameOverEvent, TileChosenEvent};
use crate::resources::{GameState, PlayerTurn};
use crate::types::{Coord, PieceSize, Player};
use bevy::prelude::{AssetServer, Commands, EventReader, EventWriter, Query, Res, ResMut};
use bevy::utils::HashSet;

pub fn tile_chosen_event_handler(
    mut tile_chosen_event_reader: EventReader<TileChosenEvent>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
    mut player_turn: ResMut<PlayerTurn>,
    q_pieces: Query<&Piece>,
) {
    for event in tile_chosen_event_reader.read() {
        let player = event.player().clone();
        let tile_coord = event.tile_coord().clone();
        let piece_size = event.piece_size().clone();
        println!(
            "Player {:?} chose tile {:?} with piece size {:?}",
            player, tile_coord, piece_size
        );
        println!("=============>>>>");

        let validation_result = validate_move(&q_pieces, &tile_coord, &piece_size);
        if !validation_result {
            game_state.set_playing();
            return;
        }

        if validate_board_for_win(&q_pieces, &player) {
            game_state.set_game_over();
            game_over_event_writer.send(GameOverEvent);
            return;
        }

        spawn_piece(
            &mut commands,
            &asset_server,
            Piece::new(piece_size, player, tile_coord),
        );
        player_turn.switch();
        player_turn.reset_chosen_piece_size();
        game_state.set_playing();
    }
}

/// check 3x3 board - horizontal, vertical, diagonal
fn validate_board_for_win(q_pieces: &Query<&Piece>, player: &Player) -> bool {
    let coords = q_pieces
        .iter()
        .filter(|piece| piece.player() == *player)
        .map(|piece| piece.coord())
        .collect::<HashSet<Coord>>();

    // check if coords has item at 0,0, 0,1, 0,2
    if coords.contains(&Coord::new(0, 0))
        && coords.contains(&Coord::new(0, 1))
        && coords.contains(&Coord::new(0, 2))
    {
        return true;
    }

    // check if coords has item at 1,0, 1,1, 1,2
    if coords.contains(&Coord::new(1, 0))
        && coords.contains(&Coord::new(1, 1))
        && coords.contains(&Coord::new(1, 2))
    {
        return true;
    }

    // check if coords has item at 2,0, 2,1, 2,2
    if coords.contains(&Coord::new(2, 0))
        && coords.contains(&Coord::new(2, 1))
        && coords.contains(&Coord::new(2, 2))
    {
        return true;
    }

    // check if coords has item at 0,0 1,1 2,2
    if coords.contains(&Coord::new(0, 0))
        && coords.contains(&Coord::new(1, 1))
        && coords.contains(&Coord::new(2, 1))
    {
        return true;
    }

    // check if coords has item at 0,2 1,1 2,0
    if coords.contains(&Coord::new(0, 2))
        && coords.contains(&Coord::new(1, 1))
        && coords.contains(&Coord::new(2, 0))
    {
        return true;
    }

    return false;
}

fn validate_move(q_pieces: &Query<&Piece>, coord: &Coord, new_value: &PieceSize) -> bool {
    let piece = q_pieces.iter().find(|piece| piece.coord() == *coord);
    println!("Found piece: {:?}", piece);

    return match piece {
        Some(piece) => piece.size() < *new_value,
        None => true,
    };
}
