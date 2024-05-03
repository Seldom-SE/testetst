use std::marker::PhantomData;

use bevy::{asset::LoadContext, prelude::*};

use crate::billboard::{Billboard, BillboardPlugin};

pub fn plug(app: &mut App) {
    app.add_plugins(BillboardPlugin::<bool>::default());
}

#[derive(Debug)]
pub struct Item(pub Handle<Billboard<bool>>);

#[derive(Debug)]
pub struct Items {
    pub items: Box<[Item]>,
}

impl Items {
    pub fn new(ctx: &mut LoadContext) -> Self {
        let path = ctx.path().to_string_lossy().to_string();

        Self {
            items: Box::new([Item(ctx.add_labeled_asset(
                format!("{path}/item_0/billboard"),
                Billboard(PhantomData),
            ))]),
        }
    }
}
