use bevy::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
                width: Val::Px(80.),
                height: Val::Px(50.),
                // This height doesn't repro the bug
                // height: Val::Px(50.),
                ..default()
            },
            ..default()
        },
    ));
}
