use bevy::prelude::*;

use crate::{
    components::tiles::{Tile, TilePath, TileType},
    resources::GameAssets,
};

#[allow(clippy::type_complexity)]
pub fn on_tile_type_changed(
    game_assets: ResMut<GameAssets>,
    mut entities: Query<
        (&mut Handle<ColorMaterial>, &Tile, &TilePath, Entity),
        Or<(Changed<Tile>, Changed<TilePath>)>,
    >,
) {
    // Less clean, didnt find an optimal way to do that.

    for (mut material, tile, path, _) in entities.iter_mut() {
        let mat = if tile.tile_type == TileType::Goal {
            game_assets.goal_tile_material.clone()
        } else if tile.tile_type == TileType::Spawner {
            game_assets.spawner_tile_material.clone()
        } else if path.is_path {
            game_assets.path_tile_material.clone()
        } else if tile.tile_type == TileType::Clear {
            game_assets.clear_tile_material.clone()
        } else {
            game_assets.blocked_tile_material.clone()
        };

        // I had this at first, but I didnt find a way to handle weird priority
        // About the fact that the path goes after some of the enum value, but not some
        // Couldnt match on the tile itself

        // let mat = if tiles_path.contains(entity) {
        //     game_assets.path_tile_material.clone()
        // } else {
        //     match tile.tile_type {
        //         TileType::Clear => game_assets.clear_tile_material.clone(),
        //         TileType::Blocked => game_assets.blocked_tile_material.clone(),
        //         TileType::Spawner => game_assets.spawner_tile_material.clone(),
        //         TileType::Goal => game_assets.goal_tile_material.clone(),
        //     }
        // };

        *material = mat;
    }
}
