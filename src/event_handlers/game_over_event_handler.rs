use crate::events::GameOverEvent;
use crate::resources::{GameState, PlayerTurn};
use crate::types::{Coord, Player};
use bevy::prelude::{
    AssetServer, Color, Commands, Entity, EventReader, Query, Res, ResMut, Text, TextBundle,
};

pub fn game_over_event_handler(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
) {
    for _ in game_over_event_reader.read() {
        todo!();
    }
}
