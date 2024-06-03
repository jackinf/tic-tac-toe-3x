use crate::actions::calculate_tile_to_world_position;
use crate::constants::{BOARD_HEIGHT_UNITS, BOARD_LAYER, BOARD_WIDTH_UNITS, CELL_SCALE};
use crate::types::Coord;
use bevy::math::Vec3;
use bevy::prelude::{default, AssetServer, Commands, Res, SpriteBundle, Transform};

pub fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    for row in 0..BOARD_HEIGHT_UNITS {
        for col in 0..BOARD_WIDTH_UNITS {
            let coord = Coord::new(row, col);
            let pos = calculate_tile_to_world_position(&coord);

            let square = if (row + col) % 2 == 0 {
                "sprites/square_light.png"
            } else {
                "sprites/square_dark.png"
            };

            commands.spawn((SpriteBundle {
                texture: asset_server.load(square),
                transform: Transform::default()
                    .with_translation(pos.extend(BOARD_LAYER))
                    .with_scale(Vec3::splat(CELL_SCALE)),
                ..default()
            },));
        }
    }
}
