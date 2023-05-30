use std::ops::RangeInclusive;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, emath::Numeric, Ui},
    EguiContexts,
};

use crate::{
    resources::{GameConfig, MapConfig},
    AppState,
};

pub fn display_ui(
    mut context: EguiContexts,
    mut map_config: ResMut<MapConfig>,
    mut game_config: ResMut<GameConfig>,
    mut state: ResMut<NextState<AppState>>,
) {
    egui::Window::new("Game Configuration").show(context.ctx_mut(), |ui| {
        ui.label("Map configuration");
        egui::Grid::new("MapConfig").show(ui, |ui| {
            slider_widget(ui, "Hex Size", &mut map_config.map_radius, 5..=100);
            ui.end_row();
        });
        ui.end_row();
        ui.label("Enemies configuration");

        let enemies_min_health = game_config.enemies_min_health;
        let enemies_max_health = game_config.enemies_max_health;

        let enemies_min_speed = game_config.enemies_min_speed;
        let enemies_max_speed = game_config.enemies_max_speed;

        egui::Grid::new("EnemiesConfig").show(ui, |ui| {
            slider_widget(ui, "Spawn Rate", &mut game_config.spawn_rate, 1..=10);
            ui.end_row();
            slider_widget(
                ui,
                "Min health",
                &mut game_config.enemies_min_health,
                1..=enemies_max_health,
            );
            ui.end_row();
            slider_widget(
                ui,
                "Max health",
                &mut game_config.enemies_max_health,
                enemies_min_health..=100,
            );
            ui.end_row();

            ui.end_row();
            slider_widget(
                ui,
                "Min speed",
                &mut game_config.enemies_min_speed,
                0.1..=enemies_max_speed,
            );
            ui.end_row();
            slider_widget(
                ui,
                "Max speed",
                &mut game_config.enemies_max_speed,
                enemies_min_speed..=10.,
            );
            ui.end_row();
        });

        ui.end_row();
        ui.label("Tower configuration");
        egui::Grid::new("TowerConfig").show(ui, |ui| {
            slider_widget(ui, "Tower damage", &mut game_config.tower_damage, 1..=100);
            ui.end_row();
            slider_widget(ui, "Tower range", &mut game_config.tower_range, 1..=20);
            ui.end_row();
        });

        if ui.add(egui::Button::new("Regenerate Board")).clicked() {
            state.set(AppState::Starting);
        }
    });

    // egui::Window::new("Tower Creation")
    //     .anchor(Align2::LEFT_BOTTOM, [0.0, 0.0])
    //     .show(context.ctx_mut(), |ui| {
    //         if ui.add(egui::Button::new("Create Tower")).clicked() {
    //             cache.placing_tower = true;
    //         }
    //     });
}

fn slider_widget<N: Numeric>(ui: &mut Ui, name: &str, value: &mut N, range: RangeInclusive<N>) {
    ui.label(name);
    ui.add(egui::Slider::new(value, range));
}
