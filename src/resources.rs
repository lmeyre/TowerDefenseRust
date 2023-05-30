use bevy::{
    prelude::{Handle, Mesh, Resource},
    sprite::ColorMaterial,
};
use hexx::{Hex, Vec2};

#[derive(Debug, Resource)]
pub struct TowerSpawn {
    pub should_spawn: bool,
    pub target: Hex,
}

impl Default for TowerSpawn {
    fn default() -> Self {
        Self {
            should_spawn: false,
            target: Hex::ZERO,
        }
    }
}

#[derive(Debug, Resource)]
pub struct MapConfig {
    pub hex_size: Vec2,
    pub map_radius: u32,
    pub budget: u32,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            hex_size: Vec2::splat(14.0),
            map_radius: 20,
            budget: 13,
        }
    }
}

#[derive(Debug, Resource)]
pub struct GameConfig {
    // Enemies
    pub spawn_rate: u64,
    pub enemies_min_health: u32,
    pub enemies_max_health: u32,

    pub enemies_min_speed: f32,
    pub enemies_max_speed: f32,
    //Tower
    pub tower_damage: u32,
    pub tower_range: u32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            spawn_rate: 2,
            enemies_min_health: 5,
            enemies_max_health: 20,
            enemies_min_speed: 0.2,
            enemies_max_speed: 3.,
            tower_damage: 10,
            tower_range: 5,
        }
    }
}

#[derive(Debug, Resource)]
pub struct GameAssets {
    //Mesh
    pub bestagone_mesh: Handle<Mesh>,
    pub circle_mesh: Handle<Mesh>,
    pub square_mesh: Handle<Mesh>,
    //Mats
    pub tower_material: Handle<ColorMaterial>,
    pub enemy_material: Handle<ColorMaterial>,
    pub clear_tile_material: Handle<ColorMaterial>,
    pub blocked_tile_material: Handle<ColorMaterial>,
    pub spawner_tile_material: Handle<ColorMaterial>,
    pub goal_tile_material: Handle<ColorMaterial>,
    pub path_tile_material: Handle<ColorMaterial>,
}
