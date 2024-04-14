use bevy::prelude::*;
use bevy_rapier3d::{
    geometry::Collider,
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, init)
        .run();
}

fn init(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    cmd.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::Y * 200.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    cmd.spawn(SpatialBundle::default()).with_children(|parent| {
        parent
            .spawn(SpatialBundle::default())
            .with_children(|parent| {
                parent
                    .spawn(SpatialBundle::default())
                    .with_children(|parent| {
                        parent
                            .spawn(SpatialBundle {
                                transform: Transform::from_xyz(3., 0., 9.)
                                    .with_scale(Vec3::new(16., 1., 1.)),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(SpatialBundle {
                                        transform: Transform::from_xyz(2.813, 0., 1.062)
                                            .with_scale(Vec3::new(1., 1., 2.125)),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(SpatialBundle::default()).with_children(
                                            |parent| {
                                                parent.spawn((
                                                    PbrBundle {
                                                        mesh: meshes.add(Cuboid::from_size(
                                                            Vec3::splat(1.),
                                                        )),
                                                        ..default()
                                                    },
                                                    Collider::cuboid(0.5, 0.5, 0.5),
                                                ));
                                            },
                                        );
                                    });
                            });
                    });
            });
    });
}
