use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .run();
}

fn init(assets: Res<AssetServer>, mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
    cmd.spawn(SpriteBundle {
        texture: assets.load("imgextracted.basis"),
        ..default()
    });
}
