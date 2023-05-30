use std::time::Duration;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use hexx::*;

use crate::{
    components::{enemies::EnemiesSpawnTimer, towers::TowerAttackTimer},
    resources::{GameAssets, GameConfig, MapConfig},
};

pub fn initialize(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map_config: ResMut<MapConfig>,
    game_config: ResMut<GameConfig>,
) {
    //Game assets
    let mesh_size = 1.;
    let game_assets = GameAssets {
        bestagone_mesh: meshes.add(get_hexagonal_mesh(map_config.hex_size)),
        circle_mesh: meshes.add(shape::Circle::new(mesh_size).into()),
        square_mesh: meshes.add(shape::Box::new(mesh_size, mesh_size, mesh_size).into()),

        clear_tile_material: materials.add(Color::WHITE.into()),
        blocked_tile_material: materials.add(Color::DARK_GRAY.into()),
        spawner_tile_material: materials.add(Color::RED.into()),
        goal_tile_material: materials.add(Color::BLUE.into()),
        path_tile_material: materials.add(Color::ORANGE.into()),
        tower_material: materials.add(Color::GREEN.into()),
        enemy_material: materials.add(Color::PURPLE.into()),
    };
    commands.insert_resource(game_assets);

    // Timer
    let timer = Timer::new(
        Duration::from_secs(game_config.spawn_rate),
        TimerMode::Repeating,
    );
    commands.spawn(EnemiesSpawnTimer(timer));
    let timer = Timer::new(Duration::from_secs(1), TimerMode::Repeating);
    commands.spawn(TowerAttackTimer(timer));
}

fn get_hexagonal_mesh(hex_size: Vec2) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(&HexLayout {
        hex_size,
        ..default()
    })
    .facing(Vec3::Z)
    .build();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}
