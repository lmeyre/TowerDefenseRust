use std::{io::stdin, thread, time::Duration};

use bevy::prelude::*;
use crossbeam_channel::*;

use crate::{components::hexgrid::HexGrid, resources::TowerSpawn, AppState};

#[derive(Debug, Resource)]
pub struct ChannelTD {
    pub receiver: Receiver<String>,
}

pub fn write_channel(sender: Sender<String>) {
    loop {
        let stdin = stdin();
        let mut buff = String::new();
        if stdin.read_line(&mut buff).is_err() {
            thread::sleep(Duration::from_millis(1000));
            continue;
        }
        let buff = buff.trim();
        let split: Vec<_> = buff.split_whitespace().collect();
        if let Some(command) = split.first() {
            if *command == "restart" && sender.try_send(String::from("restart")).is_ok() {
                // Happyness
            } else if *command == "tower" {
                if let Some(position) = split.get(1) {
                    if let Some((x, y)) = position.split_once(',') {
                        if let (Some(x), Some(y)) = (x.parse::<i32>().ok(), y.parse::<i32>().ok()) {
                            if sender.try_send(format!("tower {},{}", x, y)).is_ok() {
                                // Happyness
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn listen_channel(
    receiver: Res<ChannelTD>,
    mut state: ResMut<NextState<AppState>>,
    mut spawn_tower: ResMut<TowerSpawn>,
    board: Query<&mut HexGrid>,
) {
    while let Ok(msg) = receiver.receiver.try_recv() {
        if msg == "restart" {
            // Should probably store this string in a const
            state.set(AppState::Starting);
        } else if msg.starts_with("tower ") {
            let tower_coordinates = msg.trim_start_matches("tower ").to_string();
            if let Some((x, y)) = parse_tower_coordinates(&tower_coordinates) {
                if let Ok(grid) = board.get_single() {
                    let pos = Vec2::new(x as f32 * 14.0, y as f32 * 14.0);
                    let hex_pos = grid.layout.world_pos_to_hex(pos);
                    spawn_tower.should_spawn = true;
                    spawn_tower.target = hex_pos;
                }
            }
        }
    }
}

fn parse_tower_coordinates(coordinates: &str) -> Option<(i32, i32)> {
    let split: Vec<_> = coordinates.split(',').collect();
    if split.len() == 2 {
        if let (Ok(x), Ok(y)) = (split[0].parse::<i32>(), split[1].parse::<i32>()) {
            return Some((x, y));
        }
    }
    None
}
