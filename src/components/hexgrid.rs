use bevy::prelude::{Component, Entity};
use hexx::{Hex, HexLayout};
use std::collections::HashMap;

#[derive(Debug, Component)]
pub struct HexGrid {
    //  This hashmap allow us to quickly access any Tile entity from a `Hex`
    pub tiles_entities: HashMap<Hex, Entity>,
    pub layout: HexLayout,
    pub tower_entities: HashMap<Hex, Entity>,
    pub spawner_entities: HashMap<Hex, Entity>,
}

pub struct Path;
