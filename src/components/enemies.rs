use bevy::{
    prelude::{Bundle, Component, Deref, DerefMut},
    time::Timer,
};
use hexx::Hex;
use rand::Rng;

use crate::resources::GameConfig;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub movement: Movement,
    pub health: Health,
    pub enemy: Enemy,
}

#[derive(Component)]
pub struct Health {
    pub health: u32,
}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub spawner_hex: Hex,
    pub current_target_index: usize,
}

#[derive(Component)]
pub struct Enemy {}

impl Health {
    pub fn get_random_health(min: u32, max: u32) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    pub fn get_size(&self, game_config: &GameConfig) -> f32 {
        if self.health == 0 {
            return 0.;
        }
        let health_range = 0..=game_config.enemies_max_health;
        let size_range = 3..=10;

        let normalized_position = (self.health - health_range.start()) as f32
            / (health_range.end() - health_range.start()) as f32;

        let size = (normalized_position * (size_range.end() - size_range.start()) as f32
            + *size_range.start() as f32)
            .round() as i32;

        size.clamp(*size_range.start(), *size_range.end()) as f32 //remove ?
    }
}

impl Movement {
    pub fn get_random_speed(min: f32, max: f32) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }
}

#[derive(Component)]
pub struct Spawner {
    pub hex: Hex,
    pub path: Vec<Hex>,
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct EnemiesSpawnTimer(pub Timer);
