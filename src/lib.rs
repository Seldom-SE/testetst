#![allow(clippy::type_complexity)]

use bevy::prelude::*;

// the `bevy_main` proc_macro generates the required boilerplate for iOS and Android
#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .run();
}

#[allow(dead_code)]
#[derive(Resource, Deref)]
struct SuperImportantImages(Vec<Handle<Image>>);

fn setup_scene(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        ImageScaleMode::Sliced(TextureSlicer {
            border: BorderRect::square(16.),
            sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1. },
            ..default()
        }),
        ImageBundle {
            image: assets.load("9slice.png").into(),
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        },
    ));
}
