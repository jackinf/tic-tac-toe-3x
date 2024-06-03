use crate::actions::calculate_tile_to_world_position;
use crate::components::Piece;
use crate::constants::{PIECE_LAYER, TILE_SIZE};
use crate::types::PieceSize;
use bevy::prelude::{default, AssetServer, Commands, Res, Sprite, SpriteBundle, Transform, Vec3};

pub fn spawn_piece(mut commands: &mut Commands, asset_server: &Res<AssetServer>, piece: Piece) {
    let coord = piece.coord();
    let player = piece.player();
    let size = piece.size();

    println!("=====");
    println!("SPAWN PIECE");
    println!(
        "Spawning piece at {:?} for player {:?} with size {:?}",
        coord, player, size
    );
    let world_pos = calculate_tile_to_world_position(&coord);
    println!("World position: {:?}", world_pos);

    let scale = match size {
        PieceSize::Small => Vec3::splat(0.5),
        PieceSize::Medium => Vec3::splat(1.0),
        PieceSize::Large => Vec3::splat(1.5),
    };

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/piece.png"),
            transform: Transform::default()
                .with_translation(world_pos.extend(PIECE_LAYER))
                .with_scale(scale),
            sprite: Sprite {
                color: player.color(),
                ..default()
            },
            ..Default::default()
        },
        piece,
    ));
}
