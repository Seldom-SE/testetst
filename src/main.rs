use bevy::{prelude::*, utils::Duration};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, subtract_time)
        .run();
}

fn subtract_time(time: Res<Time<Real>>) {
    if let Some(last_update) = time.last_update() {
        info!("{:?}", last_update - Duration::from_secs(60));
    }
}
