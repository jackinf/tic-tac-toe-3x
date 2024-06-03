use crate::actions::calculate_tile_to_world_position;
use crate::components::Piece;
use crate::constants::{PIECE_LAYER, PIECE_SIZE_L, PIECE_SIZE_M, PIECE_SIZE_S};
use crate::types::PieceSize;
use bevy::prelude::{default, AssetServer, Commands, Res, Sprite, SpriteBundle, Transform, Vec3};

pub fn spawn_piece(commands: &mut Commands, asset_server: &Res<AssetServer>, piece: Piece) {
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
        PieceSize::Small => Vec3::splat(PIECE_SIZE_S),
        PieceSize::Medium => Vec3::splat(PIECE_SIZE_M),
        PieceSize::Large => Vec3::splat(PIECE_SIZE_L),
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
