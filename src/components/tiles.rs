use bevy::prelude::Component;
use std::fmt;

#[derive(Component, Clone)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Component)]
// Need bool, this isnt enough, cause we need to see when a tile has stopped being a path, just the component removed wont trigger a system
pub struct TilePath {
    pub is_path: bool, // Cant put the bool in Tile or changing it will trigger tile update and will do that forever (cause updating tile first set them to not be path, then find the path again)
}

#[derive(Component)]
pub struct DamageArea {
    pub damage: u32,
}

#[derive(Clone, PartialEq, Copy)]
pub enum TileType {
    Clear,
    Blocked,
    Spawner,
    Goal,
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the formatting logic here
        match self {
            TileType::Clear => write!(f, "Clear"),
            TileType::Blocked => write!(f, "Blocked"),
            TileType::Spawner => write!(f, "Spawner"),
            TileType::Goal => write!(f, "Goal"),
        }
    }
}

impl TileType {
    pub fn get_cost(&self) -> u32 {
        match self {
            TileType::Clear => 1,
            TileType::Blocked => 1000,
            TileType::Spawner => 1,
            TileType::Goal => 1,
        }
    }

    pub fn is_valid_spawn(&self) -> bool {
        !matches!(self, TileType::Spawner | TileType::Goal)
    }
}
