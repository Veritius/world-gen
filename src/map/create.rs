use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[cfg(not(feature="graphics"))]
use bevy_ecs_tilemap::StandardTilemapBundle;

pub fn create_tile_map(
    mut commands: Commands,
    size: TilemapSize,
) {
    let tile_storage = TileStorage::empty(size);
    let tilemap_entity = commands.spawn_empty().id();
    // let tilemap_id = TilemapId(tilemap_entity);
    let map_type = TilemapType::Hexagon(HexCoordSystem::Row);
    let tile_size = TilemapTileSize { x: 15.0, y: 17.0 };
    let grid_size = TilemapGridSize { x: 15.0, y: 17.0 };

    #[cfg(feature="graphics")] {
        commands.entity(tilemap_entity).insert(TilemapBundle {
            storage: tile_storage,
            map_type,
            tile_size,
            grid_size,
            size,
            ..default()
        });
    }

    #[cfg(not(feature="graphics"))] {
        commands.entity(tilemap_entity).insert(StandardTilemapBundle {
            storage: tile_storage,
            map_type,
            tile_size,
            grid_size,
            size,
            ..default()
        });
    }
}