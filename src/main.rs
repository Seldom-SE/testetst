use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, init)
        .run();
}

fn init(mut commands: Commands) {
    for x in -16..16 {
        for z in -8..8 {
            for y in -8..0 {
                commands.spawn((
                    SpatialBundle {
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..default()
                    },
                    RigidBody::Static,
                    Collider::cuboid(1., 1., 1.),
                ));
            }
        }
    }
}
