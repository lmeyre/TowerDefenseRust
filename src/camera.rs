use bevy::prelude::{Camera2dBundle, Commands};

/// 3D Orthogrpahic camera setup
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
