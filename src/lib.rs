use std::time::Duration;

use bevy::{color::palettes::css::DARK_GRAY, prelude::*, winit::WinitSettings};

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(bevy::window::Window {
                resizable: false,
                mode: bevy::window::WindowMode::Fullscreen, // (MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Reactive {
                wait: Duration::from_secs_f32(1.0 / 60.0),
                react_to_device_events: false,
                react_to_user_events: false,
                react_to_window_events: false,
            },
            ..Default::default()
        })
        .add_systems(Startup, init)
        .add_systems(Update, handle_press)
        .run();
}

#[derive(Resource, Deref)]
struct MySound(Handle<AudioSource>);

fn init(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(MySound(asset_server.load("sounds/sound.ogg")));
}

fn handle_press(
    my_sound: Res<MySound>,
    mut clear_color: ResMut<ClearColor>,
    touches: Res<Touches>,
    mut cmds: Commands,
) {
    if touches.any_just_pressed() {
        cmds.spawn(AudioBundle {
            source: (*my_sound).clone(),
            ..default()
        });
    }

    **clear_color = if touches.first_pressed_position().is_some() {
        Color::WHITE
    } else {
        DARK_GRAY.into()
    };
}
