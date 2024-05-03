use std::marker::PhantomData;

use bevy::{asset::LoadContext, prelude::*};

use crate::billboard::{Billboard, BillboardPlugin};

pub fn plug(app: &mut App) {
    app.add_plugins(BillboardPlugin::<()>::default());
}

#[derive(Debug)]
pub struct Enemy {
    pub billboard: Handle<Billboard<()>>,
}

#[derive(Debug)]
pub struct Enemies(pub Box<[Enemy]>);

impl Enemies {
    pub fn new(ctx: &mut LoadContext) -> Self {
        let path = ctx.path().to_string_lossy().to_string();

        Self(Box::new([Enemy {
            billboard: ctx
                .add_labeled_asset(format!("{path}/item_0/billboard"), Billboard(PhantomData)),
        }]))
    }
}
