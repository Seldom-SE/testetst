use bevy::{log::LogPlugin, prelude::*};

fn main() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin::default()));
    let world = app.world_mut();
    let parent = world.spawn(()).id();
    world.spawn_batch([ChildOf(parent), ChildOf(parent)]);

    let ids = world
        .iter_entities()
        .map(|entity| entity.id())
        .collect::<Vec<_>>();

    for id in ids {
        world.commands().entity(id).log_components();
    }

    world.flush();
}
