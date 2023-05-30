use bevy::{prelude::*, time::Timer};
use hexx::Hex;

#[derive(Component)]
pub struct Tower {
    pub hex: Hex,
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct TowerAttackTimer(pub Timer);
