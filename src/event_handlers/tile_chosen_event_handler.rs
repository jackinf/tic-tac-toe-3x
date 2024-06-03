use crate::actions::{spawn_piece, validate_board_for_win, validate_if_move_is_valid};
use crate::components::Piece;
use crate::events::{GameOverEvent, TileChosenEvent};
use crate::resources::PlayerTurn;
use crate::states::AppState;
use bevy::prelude::{
    AssetServer, Commands, EventReader, EventWriter, NextState, Query, Res, ResMut,
};

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

        let mut old_pieces: Vec<Piece> = q_pieces.iter().cloned().collect::<Vec<Piece>>();
        let new_piece = Piece::new(piece_size.clone(), player, coord);
        if !validate_if_move_is_valid(&old_pieces, &new_piece) {
            println!("Invalid move");
            app_state.set(AppState::Playing);
            return;
        }

        spawn_piece(&mut commands, &asset_server, new_piece.clone());

        old_pieces.push(new_piece.clone());
        if validate_board_for_win(&old_pieces, &player) {
            app_state.set(AppState::GameOver);
            game_over_event_writer.send(GameOverEvent::new(player.clone()));
            return;
        }

        player_turn.switch();
        player_turn.reset_chosen_piece_size();
        app_state.set(AppState::Playing);
    }
}
