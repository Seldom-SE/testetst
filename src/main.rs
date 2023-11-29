use bevy::prelude::*;

fn detect(mut window_moved: EventReader<WindowMoved>) {
    for _ in window_moved.read() {
        info!("Moved!");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, detect)
        .run();
}
