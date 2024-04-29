use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EditorPlugin::default()))
        .add_systems(Startup, init)
        .add_systems(Update, insert_physics)
        .run();
}

fn init(assets: Res<AssetServer>, mut cmd: Commands) {
    cmd.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::Y * 200.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    cmd.spawn(SpatialBundle::default()).with_children(|parent| {
        parent.spawn(SceneBundle {
            scene: assets.load("level.glb#Scene0"),
            ..default()
        });
    });
}

fn insert_physics(names: Query<(&Name, Entity), Added<Name>>, mut cmd: Commands) {
    names.iter().for_each(|(name, entity)| {
        if &**name == "Plane" {
            cmd.entity(entity)
                .insert((RigidBody::Fixed, Collider::cuboid(1., 1., 1.)));
        }
    });
}
