use std::marker::PhantomData;

use bevy::prelude::*;

pub struct BillboardPlugin<R>(PhantomData<R>);

impl<R> Default for BillboardPlugin<R> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<R: 'static + TypePath + Send + Sync> Plugin for BillboardPlugin<R> {
    fn build(&self, app: &mut App) {
        app.init_asset::<Billboard<R>>();
    }
}

#[derive(Asset, Reflect, Debug)]
pub struct Billboard<R: TypePath + Send + Sync>(pub PhantomData<R>);
