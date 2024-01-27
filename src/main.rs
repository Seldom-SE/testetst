use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<MyEvent>()
        .add_systems(Startup, init)
        .add_systems(Update, (send, run_sys).chain())
        .run();
}

fn init(world: &mut World) {
    let mut sys = IntoSystem::into_system(read);
    sys.initialize(world);
    world.insert_resource(Sys(Box::new(sys)));
}

fn send(mut write: EventWriter<MyEvent>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        write.send(MyEvent);
    }
}

#[derive(Resource)]
struct Sys(Box<dyn System<In = (), Out = ()>>);

fn run_sys(world: &mut World) {
    // Temporarily taking ownership of `Sys`
    let Sys(mut sys) = world.remove_resource().unwrap();
    // sys.initialize(world);
    // sys.set_last_run(world.change_tick());
    sys.run((), world);
    world.insert_resource(Sys(sys));
}

#[derive(Event)]
struct MyEvent;

fn read(mut has_run: Local<bool>, mut read: EventReader<MyEvent>) {
    if !*has_run {
        read.read().last();
        *has_run = true;
        return;
    }

    if read.read().last().is_some() {
        info!("Read event");
    }
}
