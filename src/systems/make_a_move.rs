use crate::actions::calculate_world_to_tile_position;
use crate::events::TileChosenEvent;
use crate::resources::PlayerTurn;
use crate::types::PieceSize;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::math::Vec3Swizzles;
use bevy::prelude::{
    Camera, EventReader, EventWriter, GlobalTransform, KeyCode, MouseButton, Query, Res, ResMut,
    Transform, Window, With,
};
use bevy::window::PrimaryWindow;

pub fn make_a_move(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut player_turn: ResMut<PlayerTurn>,
    mut tile_chosen_event_writer: EventWriter<TileChosenEvent>,
    mut key_button_events: EventReader<KeyboardInput>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {
    for key_button_event in key_button_events.read() {
        match key_button_event.key_code {
            KeyCode::KeyQ => player_turn.set_chosen_piece_size(PieceSize::Small),
            KeyCode::KeyW => player_turn.set_chosen_piece_size(PieceSize::Medium),
            KeyCode::KeyE => player_turn.set_chosen_piece_size(PieceSize::Large),
            KeyCode::Escape => player_turn.reset_chosen_piece_size(),
            _ => {}
        }
    }

    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.xy())
    {
        println!("World position: {:?}", world_position);
        let tile = calculate_world_to_tile_position(&world_position);
        println!("Tile chosen: {:?}", tile);

        for event in mouse_button_input_events.read() {
            if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
                if player_turn.chosen_piece_size().is_none() {
                    println!("Choose a piece first");
                    continue;
                }

                if tile.is_none() {
                    println!("Out of bounds");
                    continue;
                }

                let tile = tile.unwrap();
                let size = player_turn.chosen_piece_size().unwrap();
                let player = player_turn.player();

                let tile_chosen_event = TileChosenEvent::new(player, tile, size);
                tile_chosen_event_writer.send(tile_chosen_event);
            }
        }
    }
}
