use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TilemapPlugin))
        .add_systems(Startup, init)
        .run();
}

const MAP_SIZE: UVec2 = UVec2::new(12, 12);
const TILE_SIZE: Vec2 = Vec2::new(100., 50.);

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        // Centering the camera
        transform: Transform::from_xyz(MAP_SIZE.x as f32 * TILE_SIZE.x / 2., 0., 0.),
        ..default()
    });

    let map_size = MAP_SIZE.into();
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    commands.entity(tilemap_entity).with_children(|parent| {
        for x in 0..MAP_SIZE.x {
            for y in 0..MAP_SIZE.y {
                let tile_pos = UVec2::new(x, y).into();

                tile_storage.set(
                    &tile_pos,
                    parent
                        .spawn(TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            ..default()
                        })
                        .id(),
                );
            }
        }
    });

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(asset_server.load("tile-iso.png")),
        tile_size: TILE_SIZE.into(),
        map_type: TilemapType::Isometric(IsoCoordSystem::Diamond),
        transform: Transform::from_xyz(50., 0., 0.),
        ..default()
    });
}
