use crate::constants::TILE_SIZE;
use crate::types::Coord;
use bevy::prelude::Vec2;

pub fn calculate_tile_to_world_position(coord: &Coord) -> Vec2 {
    let x = coord.x as f32 * TILE_SIZE;
    let y = coord.y as f32 * TILE_SIZE;
    Vec2::new(x, y)
}

pub fn calculate_world_to_tile_position(world_pos: &Vec2) -> Option<Coord> {
    let offset = 100.; // why 100?
    let world_x = world_pos.x + offset;
    let world_y = world_pos.y + offset;
    if world_x < 0. || world_x > 600. || world_y < 0. || world_y > 600. {
        return None;
    }

    let x = ((world_x) / TILE_SIZE).floor() as usize;
    let y = ((world_y) / TILE_SIZE).floor() as usize;
    Some(Coord::new(x, y))
}
