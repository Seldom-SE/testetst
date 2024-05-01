use std::marker::PhantomData;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

struct MyParam<T>(PhantomData<T>);

fn my_sys() {}

fn testetts(param: MyParam<typeof(my_sys)>) {}
