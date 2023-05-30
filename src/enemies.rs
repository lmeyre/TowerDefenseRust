use bevy::prelude::*;
use hexx::{algorithms::a_star, Hex};

use crate::components::tiles::TilePath;
use crate::{
    components::{
        enemies::{EnemiesSpawnTimer, Enemy, EnemyBundle, Health, Movement, Spawner},
        hexgrid::HexGrid,
        tiles::Tile,
    },
    resources::{GameAssets, GameConfig},
};

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Query<&mut EnemiesSpawnTimer>,
    game_config: Res<GameConfig>,
    game_assets: Res<GameAssets>,
    spawners: Query<&Spawner>,
    grid: Query<(&HexGrid, Entity)>,
) {
    if let Ok(mut timer) = timer.get_single_mut() {
        if timer.tick(time.delta()).finished() {
            timer.reset();
            if let Ok((grid, board_entity)) = grid.get_single() {
                for spawner in spawners.iter() {
                    let health: Health = Health {
                        health: Health::get_random_health(
                            game_config.enemies_min_health,
                            game_config.enemies_max_health,
                        ),
                    };
                    let size = health.get_size(game_config.as_ref());
                    let position = grid.layout.hex_to_world_pos(spawner.hex);
                    commands
                        .spawn(ColorMesh2dBundle {
                            mesh: game_assets.circle_mesh.clone().into(),
                            material: game_assets.enemy_material.clone(),
                            transform: Transform {
                                translation: (Vec3 {
                                    x: (position.x),
                                    y: (position.y),
                                    z: (0.3),
                                }),
                                scale: Vec3 {
                                    x: size,
                                    y: size,
                                    z: size,
                                },
                                ..default()
                            },
                            ..default()
                        })
                        .insert(EnemyBundle {
                            movement: Movement {
                                speed: Movement::get_random_speed(
                                    game_config.enemies_min_speed,
                                    game_config.enemies_max_speed,
                                ),
                                spawner_hex: spawner.hex,
                                current_target_index: 0,
                            },
                            health,
                            enemy: Enemy {},
                        })
                        .set_parent(board_entity);
                }
            }
        }
    }
}

pub fn on_damage_taken(
    mut commands: Commands,
    mut damaged: Query<(&Health, Entity, &mut Transform), Changed<Health>>,
    game_config: Res<GameConfig>,
    grid: Query<Entity, With<HexGrid>>,
) {
    if let Ok(board) = grid.get_single() {
        for (health, entity, mut transform) in damaged.iter_mut() {
            if health.health == 0 {
                commands.entity(board).remove_children(&[entity]);
                commands.entity(entity).despawn();
                continue;
            }

            let size = health.get_size(game_config.as_ref());
            transform.scale = Vec3 {
                x: size,
                y: size,
                z: size,
            };
        }
    }
}

// Could make them update their position if path are updated
// Not complicated but will take more time again
// I dont feel like it's necessary, can add it if needed
pub fn move_enemies(
    mut commands: Commands,
    mut enemies: Query<(&mut Movement, &mut Transform, Entity)>,
    grid: Query<&HexGrid>,
    spawners: Query<&Spawner>,
    time: Res<Time>,
) {
    //Enemies will look where they are in the path of their spawner
    //They will have a target hex, once distance is small, they go to the next
    //if let Ok(grid) = grid.get_single() {
    let grid = match grid.get_single() {
        Ok(g) => g,
        Err(_) => return,
    };
    for (mut movement, mut transform, entity) in enemies.iter_mut() {
        if let Some(spawner_entity) = grid.spawner_entities.get(&movement.spawner_hex) {
            if let Ok(spawner) = spawners.get(*spawner_entity) {
                if let Some(target_hex) = spawner.path.get(movement.current_target_index) {
                    let target_pos = grid.layout.hex_to_world_pos(*target_hex).extend(0.);
                    let distance = transform.translation.distance(target_pos);
                    // Change target when tile is reached
                    if distance < 1. {
                        // If goal is reached
                        if *target_hex == Hex::ZERO {
                            commands.entity(entity).despawn();
                        } else {
                            movement.current_target_index += 1;
                        }
                    }

                    //Move the entites
                    let translation = &mut transform.translation;
                    *translation =
                        translation.lerp(target_pos, movement.speed * time.delta_seconds());
                }
            }
        }
    }
}

pub fn update_paths(
    mut spawners: Query<&mut Spawner>,
    grid: Query<&HexGrid>,
    mut tiles: Query<(&mut TilePath, &Tile)>,
    changed_tiles: Query<Changed<Tile>>,
) {
    if changed_tiles.is_empty() {
        return;
    }

    if let Ok(grid) = grid.get_single() {
        for (mut path, _) in tiles.iter_mut() {
            path.is_path = false;
        }
        for mut spawner in spawners.iter_mut() {
            if let Some(mut path) = a_star(spawner.hex, Hex::ZERO, |hex| {
                if let Some(entity) = grid.tiles_entities.get(&hex) {
                    if let Ok((_, tile)) = tiles.get(*entity) {
                        Some(tile.tile_type.get_cost())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }) {
                path.remove(0);
                for spawner_path_tile in path.clone() {
                    if let Some(entity) = grid.tiles_entities.get(&spawner_path_tile) {
                        if let Ok((mut path, _)) = tiles.get_mut(*entity) {
                            path.is_path = true;
                        }
                    }
                }
                spawner.path = path;
            }
        }
    }
}

// Workaround, this is probably the biggest problem that I had on the project
// I had to make a ParamSet to stop crash and conflicts in the Query
// But making so made me unable to give the tiles to the function used by a_star
// Since it didnt wanted it to be mut (But I had no choice of having the paramset mut)
// Became a rabbit hole where I tried more and more fix, then more fix to fix problems created by said "fixs"
// In the end I just did the HashMap on Benoit's advices even tho its kinda brute force
// Interested which path i should have taken

// pub fn pre_update_paths(
//     mut tiles: ParamSet<(Query<(&TilePath, Entity, &mut Tile)>, Query<Changed<Tile>>)>,
// ) {
//     if tiles.p1().is_empty() {
//         return;
//     }
//     for (_, _, mut tile) in tiles.p0().iter_mut() {
//         info!("Setting tile to false");
//         tile.is_path = false;
//     }
// }

// pub fn on_tile_path_updated(mut tiles: Query<&mut Tile, Added<TilePath>>) {
//     for mut tile in tiles.iter_mut() {
//         info!("Setting tile to true");
//         tile.is_path = true;
//     }
// }

// pub fn update_paths(
//     mut commands: Commands,
//     mut spawners: Query<&mut Spawner>,
//     grid: Query<&HexGrid>,
//     mut tiles: ParamSet<(
//         Query<(&TilePath, Entity, &mut Tile)>,
//         Query<Changed<Tile>>,
//         Query<&Tile>,
//     )>,
// ) {
//     if tiles.p1().is_empty() {
//         return;
//     }

//     if let Ok(grid) = grid.get_single() {
//         for (_, entity, mut tile) in tiles.p0().iter_mut() {
//             commands.entity(entity).remove::<TilePath>();
//             tile.is_path = false;
//         }
//         for mut spawner in spawners.iter_mut() {
//             if let Some(mut path) = a_star(spawner.hex, Hex::ZERO, |hex| {
//                 if let Some(entity) = grid.tiles_entities.get(&hex) {
//                     if let Ok(tile) = tiles.p2().get(*entity) {
//                         Some(tile.tile_type.get_cost())
//                     } else {
//                         None
//                     }
//                 } else {
//                     None
//                 }
//             }) {
//                 path.remove(0);
//                 for spawner_path_tile in path.clone() {
//                     if let Some(entity) = grid.tiles_entities.get(&spawner_path_tile) {
//                         commands.entity(*entity).insert(TilePath {});
//                     }
//                 }
//                 spawner.path = path;
//             }
//         }
//     }
// }
