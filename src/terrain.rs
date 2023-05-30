use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    components::{
        hexgrid::HexGrid,
        tiles::{Tile, TileType},
    },
    input::LeftClickEvent,
};

pub fn change_terrain(
    mut left_click_event: EventReader<LeftClickEvent>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut board: Query<&mut HexGrid>,
    mut tiles: Query<&mut Tile>,
) {
    if left_click_event.iter().last().is_some() {
        if let Ok(windows) = windows.get_single() {
            if let Some(pos) = windows.cursor_position() {
                if let Ok(grid) = board.get_single_mut() {
                    let pos = pos - Vec2::new(windows.width(), windows.height()) / 2.0;

                    let hex_pos = grid.layout.world_pos_to_hex(pos);
                    if let Some(tile_entity) = grid.tiles_entities.get(&hex_pos) {
                        if let Ok(mut tile) = tiles.get_mut(*tile_entity) {
                            tile.tile_type = match tile.tile_type {
                                TileType::Clear => TileType::Blocked,
                                TileType::Blocked => TileType::Clear,
                                _ => tile.tile_type,
                            }
                        }
                    }
                }
            }
        }
    }
}
