use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    components::{
        enemies::Health,
        hexgrid::HexGrid,
        tiles::{DamageArea, Tile},
        towers::{Tower, TowerAttackTimer},
    },
    input::RightClickEvent,
    resources::{GameAssets, GameConfig, TowerSpawn},
};

pub fn spawn_tower_click(
    mut right_click_event: EventReader<RightClickEvent>,
    mut spawn_tower: ResMut<TowerSpawn>,
    windows: Query<&Window, With<PrimaryWindow>>,
    board: Query<&mut HexGrid>,
) {
    if right_click_event.iter().last().is_some() {
        if let Ok(windows) = windows.get_single() {
            if let Some(pos) = windows.cursor_position() {
                if let Ok(grid) = board.get_single() {
                    let pos = pos - Vec2::new(windows.width(), windows.height()) / 2.0;
                    let hex_pos = grid.layout.world_pos_to_hex(pos);
                    spawn_tower.should_spawn = true;
                    spawn_tower.target = hex_pos;
                }
            }
        }
    }
}

pub fn spawn_tower(
    mut commands: Commands,
    mut spawn_tower: ResMut<TowerSpawn>,
    mut board: Query<(&mut HexGrid, Entity)>,
    tiles: Query<&Tile>,
    game_assets: Res<GameAssets>,
) {
    if spawn_tower.should_spawn {
        if let Ok((mut grid, board_entity)) = board.get_single_mut() {
            let tile_entity = grid.tiles_entities.get(&spawn_tower.target);

            if let Some(tile_entity) = tile_entity {
                let tile = tiles.get(*tile_entity);
                if let Ok(t) = tile {
                    if t.tile_type.is_valid_spawn()
                        && !grid.tower_entities.contains_key(&spawn_tower.target)
                    {
                        let position = grid.layout.hex_to_world_pos(spawn_tower.target);
                        let tower_entity = commands
                            .spawn(ColorMesh2dBundle {
                                mesh: game_assets.square_mesh.clone().into(),
                                material: game_assets.tower_material.clone(),
                                transform: Transform {
                                    translation: (Vec3 {
                                        x: (position.x),
                                        y: (position.y),
                                        z: (0.2),
                                    }),
                                    scale: Vec3 {
                                        x: (10.),
                                        y: (10.),
                                        z: (10.),
                                    },
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(Tower {
                                hex: spawn_tower.target,
                            })
                            .set_parent(board_entity)
                            .id();
                        grid.tower_entities.insert(spawn_tower.target, tower_entity);
                    } else {
                        info!("{}", t.tile_type.is_valid_spawn());
                        info!("{}", t.tile_type);
                        info!("{}", !grid.tower_entities.contains_key(&spawn_tower.target));
                    }
                }
            }
        }
        spawn_tower.should_spawn = false;
    }
}

pub fn on_tower_spawned(
    mut commands: Commands,
    new_towers: Query<&Tower, Added<Tower>>,
    mut damage_tiles: Query<&mut DamageArea>,
    game_config: Res<GameConfig>,
    grid: Query<&HexGrid>,
) {
    if let Ok(grid) = grid.get_single() {
        for tower in new_towers.iter() {
            tower
                .hex
                .spiral_range(0..=game_config.tower_range)
                .for_each(|h| {
                    if let Some(hex_entity) = grid.tiles_entities.get(&h) {
                        if !damage_tiles.contains(*hex_entity) {
                            commands.entity(*hex_entity).insert(DamageArea {
                                damage: game_config.tower_damage,
                            });
                        }
                        if let Ok(mut damage_tile) = damage_tiles.get_mut(*hex_entity) {
                            damage_tile.damage += game_config.tower_damage;
                        }
                    }
                });
        }
    }
}

// Deal damage to entities
pub fn damage_entities(
    mut entities: Query<(&mut Health, &Transform)>,
    damaging_tiles: Query<&DamageArea>,
    grid: Query<&HexGrid>,
    mut timer: Query<&mut TowerAttackTimer>,
    time: Res<Time>,
) {
    if let Ok(mut timer) = timer.get_single_mut() {
        if timer.tick(time.delta()).finished() {
            timer.reset();
            for (mut health, position) in entities.iter_mut() {
                if let Ok(grid) = grid.get_single() {
                    // Getting the hex entity at the position of the enemy
                    let hex = grid.layout.world_pos_to_hex(Vec2 {
                        x: position.translation.x,
                        y: position.translation.y,
                    });
                    // Get tile entity
                    if let Some(tile_entity) = grid.tiles_entities.get(&hex) {
                        // If it carry damage, apply it to the enemy
                        if let Ok(damaging_tile) = damaging_tiles.get(*tile_entity) {
                            health.health = health.health.saturating_sub(damaging_tile.damage);
                        }
                    }
                }
            }
        }
    }
}
