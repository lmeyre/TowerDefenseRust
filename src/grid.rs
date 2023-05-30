use std::collections::HashMap;

use crate::{
    components::{enemies::Spawner, hexgrid::HexGrid, tiles::*},
    resources::GameAssets,
    AppState,
};
use bevy::prelude::*;
use hexx::{DiagonalDirection, Hex, HexLayout};
use rand::Rng;

use crate::resources::MapConfig;

pub fn setup_grid(
    map_config: ResMut<MapConfig>,
    mut commands: Commands,
    game_assets: ResMut<GameAssets>,
    grid: Query<(&HexGrid, Entity)>,
) {
    if let Ok((_, board_entity)) = grid.get_single() {
        commands.entity(board_entity).despawn_recursive();
    }

    let layout = HexLayout {
        hex_size: map_config.hex_size,
        ..default()
    };

    let mut rng = rand::thread_rng();

    let board_entity = commands
        .spawn(SpatialBundle {
            visibility: Visibility::Visible,
            ..Default::default()
        })
        .id();

    let tiles_entities: HashMap<_, _> = Hex::ZERO
        .spiral_range(0..=map_config.map_radius)
        .enumerate()
        .map(|(_i, coord)| {
            let tile_type = if coord == Hex::ZERO {
                TileType::Goal
            } else {
                get_random_tile_type(rng.gen_range(0..=1))
            };
            let pos = layout.hex_to_world_pos(coord);
            let entity: Entity = commands
                .spawn(ColorMesh2dBundle {
                    mesh: game_assets.bestagone_mesh.clone().into(),
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::splat(0.95)),
                    // No material here :)
                    ..default()
                })
                .insert(Tile { tile_type })
                .insert(TilePath { is_path: false })
                .set_parent(board_entity)
                .id();
            (coord, entity)
        })
        .collect();

    commands.entity(board_entity).insert(HexGrid {
        tiles_entities,
        layout,
        tower_entities: HashMap::new(),
        spawner_entities: HashMap::new(),
    });
}

fn get_random_tile_type(random: u8) -> TileType {
    match random {
        0 => TileType::Clear,
        1 => TileType::Blocked,
        _ => unreachable!(),
    }
}

pub fn setup_spawners(
    mut commands: Commands,
    map_config: ResMut<MapConfig>,
    mut grid: Query<(&mut HexGrid, Entity)>,
    mut tiles: Query<&mut Tile>,
    mut state: ResMut<NextState<AppState>>,
) {
    let mut rng = rand::thread_rng();
    if let Ok((mut grid, board_entity)) = grid.get_single_mut() {
        for direction in DiagonalDirection::ALL_DIRECTIONS {
            let mut hex_iterator = Hex::ZERO.ring_edge(map_config.map_radius, direction);
            let index = rng.gen_range(0..hex_iterator.len());
            if let Some(spawner_hex) = hex_iterator.nth(index) {
                if let Some(tile_entity) = grid.tiles_entities.get(&spawner_hex) {
                    let spawner_id = commands
                        .entity(*tile_entity)
                        .insert(Spawner {
                            hex: spawner_hex,
                            path: Vec::new(),
                        })
                        .set_parent(board_entity)
                        .id();
                    grid.spawner_entities.insert(spawner_hex, spawner_id);
                    if let Some(tile_entity) = grid.tiles_entities.get(&spawner_hex) {
                        if let Ok(mut tile) = tiles.get_mut(*tile_entity) {
                            tile.tile_type = TileType::Spawner;
                        }
                    }
                }
            }
        }
    }
    state.set(AppState::Running);
}
