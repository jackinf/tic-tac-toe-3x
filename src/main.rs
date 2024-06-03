use crate::event_handlers::{game_over_event_handler, tile_chosen_event_handler};
use crate::events::{GameOverEvent, TileChosenEvent};
use crate::resources::{GameState, PlayerTurn};
use crate::systems::{make_a_move, spawn_board};
use bevy::prelude::{
    default, App, Camera2dBundle, Commands, FixedUpdate, IntoSystemConfigs, PreStartup, Transform,
    Update, Vec3,
};

fn main() {
    App::new()
        .add_plugins(bevy::DefaultPlugins)
        .add_systems(PreStartup, (spawn_camera, spawn_board).chain())
        .add_systems(Update, make_a_move)
        .add_event::<GameOverEvent>()
        .add_event::<TileChosenEvent>()
        .add_systems(Update, (game_over_event_handler, tile_chosen_event_handler))
        .insert_resource(GameState::new())
        .insert_resource(PlayerTurn::new())
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::default().with_translation(Vec3::new(200., 200., 1000.)),
        ..default()
    });
}

mod actions;
mod components;
mod constants;
mod event_handlers;
mod events;
mod resources;
mod systems;
mod types;
