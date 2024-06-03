use crate::event_handlers::{game_over_event_handler, tile_chosen_event_handler};
use crate::events::{GameOverEvent, TileChosenEvent};
use crate::resources::PlayerTurn;
use crate::states::AppState;
use crate::systems::{make_a_move, spawn_board};
use bevy::prelude::{
    default, in_state, App, Camera2dBundle, Commands, IntoSystemConfigs, PreStartup, Transform,
    Update, Vec3,
};

fn main() {
    App::new()
        .add_plugins(bevy::DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(PreStartup, (spawn_camera, spawn_board).chain())
        .add_systems(Update, make_a_move.run_if(in_state(AppState::Playing)))
        .add_event::<GameOverEvent>()
        .add_event::<TileChosenEvent>()
        .add_systems(Update, (game_over_event_handler, tile_chosen_event_handler))
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
mod states;
mod systems;
mod types;
