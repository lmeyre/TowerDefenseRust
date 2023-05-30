mod camera;
mod grid;
mod input;
mod resources;
mod ui;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use channel::{write_channel, ChannelTD};
use input::{LeftClickEvent, RightClickEvent};
mod channel;
pub mod components;
mod enemies;
mod initialize;
mod terrain;
mod towers;
mod visual;
use crossbeam_channel::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Starting,
    Running,
}

pub fn main() {
    let (tx, rx) = unbounded();

    std::thread::spawn(move || {
        write_channel(tx);
    });
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_500.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ChannelTD { receiver: rx })
        .add_plugin(EguiPlugin)
        //Events
        .add_event::<LeftClickEvent>()
        .add_event::<RightClickEvent>()
        // Resources
        .init_resource::<resources::MapConfig>()
        .init_resource::<resources::GameConfig>()
        .init_resource::<resources::TowerSpawn>()
        // Startup Systems
        .add_startup_systems((initialize::initialize,).chain())
        .add_startup_system(camera::setup_camera)
        // State
        .add_state::<AppState>()
        // General systems
        .add_systems(
            (
                enemies::update_paths,
                // enemies::pre_update_paths,
                // enemies::on_tile_path_updated,
            )
                .chain(),
        )
        // Starting State systems
        .add_systems(
            (
                grid::setup_grid,
                apply_system_buffers,
                grid::setup_spawners,
                apply_system_buffers,
                enemies::update_paths,
            )
                .chain()
                .in_schedule(OnEnter(AppState::Starting)),
        )
        // Running State Systems
        .add_systems(
            (
                channel::listen_channel,
                input::handle_input,
                enemies::update_paths,
                visual::on_tile_type_changed,
                towers::spawn_tower,
                enemies::spawn_enemies,
                towers::damage_entities,
                ui::display_ui,
                towers::spawn_tower_click,
                towers::on_tower_spawned,
                terrain::change_terrain,
                enemies::move_enemies,
                enemies::on_damage_taken,
            )
                .in_set(OnUpdate(AppState::Running)),
        )
        .run();
}
