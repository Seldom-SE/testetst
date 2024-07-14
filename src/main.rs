use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            mode: AssetMode::Processed,
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource)]
#[allow(dead_code)]
struct MyHandle(Handle<Image>);

fn setup(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.insert_resource(MyHandle(assets.load::<Image>("square.png")));
}
