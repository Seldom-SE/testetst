use bevy::{prelude::*, window::WindowResolution};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(400., 196.),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        ImageScaleMode::Sliced(TextureSlicer {
            border: BorderRect::square(16.),
            // Bug also occurs with `SliceScaleMode::Stretch`
            sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1. },
            ..default()
        }),
        ImageBundle {
            image: assets.load("9slice.png").into(),
            style: Style {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                width: Val::Percent(20.),
                height: Val::Percent(20.),
                ..default()
            },
            ..default()
        },
    ));
}
