use bevy::prelude::*;
use seldom_pixel::prelude::*;

#[px_layer]
struct Layer;

fn animation_bundle(sprites: &mut PxAssets<PxSprite>) -> impl Bundle {
    (
        PxSpriteBundle::<Layer> {
            sprite: sprites.load_animated("sprite.png", 8),
            ..default()
        },
        PxAnimationBundle {
            on_finish: PxAnimationFinishBehavior::Mark,
            ..default()
        },
    )
}

fn init(mut commands: Commands, mut sprites: PxAssets<PxSprite>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((animation_bundle(&mut sprites), PxSubPosition::default()));
}

fn assign_animations(
    mut commands: Commands,
    mut sprites: PxAssets<PxSprite>,
    animations: Query<Entity, With<PxAnimationFinished>>,
    keys: Res<Input<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    for entity in &animations {
        commands
            .entity(entity)
            .insert((animation_bundle(&mut sprites),))
            .remove::<PxAnimationFinished>();
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: Vec2::splat(512.).into(),
                    ..default()
                }),
                ..default()
            }),
            PxPlugin::<Layer>::new(UVec2::splat(16), "palette.png".into()),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, init)
        .add_systems(Update, assign_animations)
        .run();
}
