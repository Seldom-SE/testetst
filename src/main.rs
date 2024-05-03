#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod asset;
mod billboard;
mod enemy;
mod item;

use asset::{Manifest, ManifestHandle};
use bevy::{log::LogPlugin, prelude::*};
#[cfg(feature = "editor")]
use bevy_editor_pls::prelude::*;
use billboard::Billboard;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    filter: "wgpu=error,naga=warn,testetst=debug".to_string(),
                    ..default()
                }),
            #[cfg(feature = "editor")]
            EditorPlugin::default(),
            (asset::plug, item::plug, enemy::plug),
        ))
        .insert_resource(Msaa::Off)
        .add_systems(Update, log)
        .run();
}

fn log(
    handle: Res<ManifestHandle>,
    manifests: Res<Assets<Manifest>>,
    billboards_1: Res<Assets<Billboard<()>>>,
    billboards_2: Res<Assets<Billboard<bool>>>,
) {
    info!("");
    info!("handle: {handle:?}");
    let manifest = manifests.get(&**handle);
    info!("manifest: {manifest:?}");
    let Some(Manifest { items, enemies }) = manifest else {
        return;
    };
    info!("billboard_1: {:?}", billboards_1.get(enemies));
    info!("billboard_2: {:?}", billboards_2.get(items));
}
