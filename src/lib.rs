#![allow(clippy::type_complexity)]

use bevy::{asset::LoadState, prelude::*};

// the `bevy_main` proc_macro generates the required boilerplate for iOS and Android
#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        mode: AssetMode::Processed,
        ..default()
    }))
    .add_systems(Startup, setup_scene)
    .add_systems(Update, check_loaded)
    .run();
}

#[allow(dead_code)]
#[derive(Resource, Deref)]
struct SuperImportantImages(Vec<Handle<Image>>);

fn setup_scene(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(SuperImportantImages(
        (0..128)
            .map(|i| assets.load(format!("{}.png", i)))
            .collect(),
    ));
}

fn check_loaded(handles: Res<SuperImportantImages>, assets: Res<AssetServer>) {
    let mut no_state = 0;
    let mut not_loaded = 0;
    let mut loading = 0;
    let mut loaded = 0;
    let mut failed = 0;

    for handle in &**handles {
        *match assets.get_load_state(handle) {
            None => &mut no_state,
            Some(LoadState::NotLoaded) => &mut not_loaded,
            Some(LoadState::Loading) => &mut loading,
            Some(LoadState::Loaded) => &mut loaded,
            Some(LoadState::Failed(_)) => &mut failed,
        } += 1;
    }

    info!("== LOAD STATES ==");
    info!("no_state   {no_state}");
    info!("not_loaded {not_loaded}");
    info!("loading    {loading}");
    info!("loaded     {loaded}");
    info!("failed     {failed}");
}
