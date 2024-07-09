use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle::default());

    cmds.spawn(NodeBundle {
        style: Style {
            align_content: AlignContent::Center,
            min_width: Val::Percent(100.),
            min_height: Val::Percent(100.),
            ..default()
        },
        ..default()
    })
    .with_children(|root| {
        root.spawn(NodeBundle {
            style: Style {
                width: Val::Px(50.),
                height: Val::Px(50.),
                ..default()
            },
            background_color: Color::WHITE.into(),
            ..default()
        });
    });
}
