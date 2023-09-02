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

// add 0.5X offset because our visible tiles spawned with center anchor
const MAP_OFFSET: Vec2 = Vec2::new(50., 0.);

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        // Centering the camera
        transform: Transform::from_translation(Vec3::new(
            MAP_SIZE.x as f32 * TILE_SIZE.x / 2.,
            0.,
            0.,
        )),
        ..default()
    });

    // init ecs_tilemap stuff
    let map_size = MAP_SIZE.into();
    let tile_size = TILE_SIZE.into();
    let grid_size = TILE_SIZE.into();
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    //  To get the squared tile size which will fit our iso coords, we should get our iso tile, upscale it twice by Y,
    // and then calculate the length of drawn tile's side, which requires to apply Pythagoras theorem for triangle
    // with sides X/2 and X/2, where X is our iso tile size by X:
    //  squared tile size we are looking for = sqrt(x^2 / 2)
    let navmap_tile_size = f32::sqrt(TILE_SIZE.x * TILE_SIZE.x / 2.);
    let navmap_tile_size = Vec2::new(navmap_tile_size, navmap_tile_size);

    // Spawn images for the tiles
    let tile_image = asset_server.load("tile-iso.png");
    let mut player_pos = default();

    // Good practice is to attach tiles to single parent for easier access
    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..MAP_SIZE.x {
            for y in 0..MAP_SIZE.y {
                let pos = UVec2::new(x, y);
                // updating player squared pos, with unoptimal but explicit way
                player_pos = UVec2::new(x, y).as_vec2() * navmap_tile_size;

                // spawning tiles, they remain invisible until we spawn tilemap
                let tile_pos = pos.into();
                let tile_entity = parent
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index: TileTextureIndex(0),
                        ..Default::default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_image),
        tile_size,
        map_type: TilemapType::Isometric(IsoCoordSystem::Diamond),
        // transform: get_tilemap_center_transform(&map_size, &grid_size, & TilemapType::Isometric(IsoCoordSystem::Diamond), 0.0),
        transform: Transform::from_xyz(MAP_OFFSET.x, MAP_OFFSET.y, 0.),
        ..Default::default()
    });
}
