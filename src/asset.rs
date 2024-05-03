use std::{convert::Infallible, marker::PhantomData};

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};

use crate::billboard::Billboard;

pub fn plug(app: &mut App) {
    app.init_asset::<Manifest>()
        .init_asset_loader::<ManifestLoader>()
        .add_systems(Startup, init);
}

#[derive(Default)]
struct ManifestLoader;

impl AssetLoader for ManifestLoader {
    type Asset = Manifest;
    type Settings = ();
    type Error = Infallible;

    fn load<'a>(
        &'a self,
        _: &'a mut Reader,
        &(): &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Manifest, Infallible>> {
        Box::pin(async move {
            let path = load_context.path().to_string_lossy().to_string();

            Ok(Manifest {
                items: load_context
                    .add_labeled_asset(format!("{path}/item_0/billboard"), Billboard(PhantomData)),
                enemies: load_context
                    .add_labeled_asset(format!("{path}/item_0/billboard"), Billboard(PhantomData)),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["manifest.ron"]
    }
}

#[derive(Asset, TypePath, Debug)]
pub struct Manifest {
    pub items: Handle<Billboard<bool>>,
    pub enemies: Handle<Billboard<()>>,
}

#[derive(Resource, Deref, Debug)]
pub struct ManifestHandle(Handle<Manifest>);

fn init(assets: Res<AssetServer>, mut cmd: Commands) {
    cmd.insert_resource(ManifestHandle(assets.load("manifest.manifest.ron")));
}
