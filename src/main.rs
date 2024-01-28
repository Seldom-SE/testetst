#![allow(clippy::type_complexity)]

use std::{f32::consts::FRAC_PI_2, iter::once};

use bevy::{
    gltf::GltfExtras,
    input::{common_conditions::input_just_pressed, mouse::MouseMotion},
    prelude::*,
    scene::SceneInstance,
    window::CursorGrabMode,
};
use bevy_xpbd_3d::{prelude::*, PhysicsStepSet};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Uncomment for `Update` behavior
            // PhysicsPlugins::default(),
            // Comment for `Update` behavior
            PhysicsPlugins::new(FixedUpdate),
        ))
        // Comment for `Update` behavior
        .insert_resource(Time::<Physics>::new_with(Physics::fixed_once_hz(64.)))
        .add_event::<WinClicked>()
        .add_systems(Startup, init)
        .add_systems(
            Update,
            (
                init_instances,
                (win_clicked, lock_cursor.run_if(on_event::<WinClicked>())).chain(),
                unlock_cursor.run_if(input_just_pressed(KeyCode::Escape)),
                look,
            ),
        )
        .add_systems(
            FixedUpdate,
            (reset_vel, move_player)
                .chain()
                .before(PhysicsStepSet::BroadPhase),
        )
        .run();
}

fn init(assets: Res<AssetServer>, mut cmd: Commands) {
    cmd.spawn((
        Level,
        SceneBundle {
            scene: assets.load("level.glb#Scene0"),
            ..default()
        },
    ));
}

#[derive(Component)]
struct Level;

#[derive(Bundle)]
pub struct PhysicsBundle {
    locked: LockedAxes,
    restitution: Restitution,
    friction: Friction,
}

impl Default for PhysicsBundle {
    fn default() -> Self {
        Self {
            locked: LockedAxes::ROTATION_LOCKED,
            restitution: Restitution {
                coefficient: 0.,
                combine_rule: CoefficientCombine::Min,
            },
            friction: Friction {
                dynamic_coefficient: 1.,
                static_coefficient: 1.,
                combine_rule: CoefficientCombine::Max,
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
enum Spawner {
    Player,
}

fn init_instances(
    levels: Query<Entity, (With<Level>, With<SceneInstance>, Added<Children>)>,
    spawners: Query<(&GltfExtras, &GlobalTransform)>,
    meshes: Query<&Handle<Mesh>>,
    mesh_assets: Res<Assets<Mesh>>,
    parents: Query<&Children>,
    mut cmd: Commands,
) {
    for level in &levels {
        for descendant in once(level).chain(parents.iter_descendants(level)) {
            if let Ok(mesh) = meshes.get(descendant) {
                cmd.entity(descendant).insert((
                    PhysicsBundle::default(),
                    RigidBody::Static,
                    Collider::trimesh_from_mesh(mesh_assets.get(mesh).unwrap()).unwrap(),
                ));
            };

            if let Ok((extras, tf)) = spawners.get(descendant) {
                if let Ok(Spawner::Player) = from_str(&extras.value) {
                    const HEIGHT: f32 = 0.4;

                    cmd.spawn((
                        Player,
                        ResetVel,
                        PointLightBundle {
                            transform: Transform::from_translation(
                                tf.translation() + Vec3::Y * HEIGHT / 2.,
                            ),
                            ..default()
                        },
                        PhysicsBundle::default(),
                        RigidBody::Dynamic,
                        Collider::capsule(HEIGHT, 0.2),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Camera3dBundle {
                                transform: Transform::from_translation(Vec3::Y),
                                ..default()
                            },
                            LookRotation::default(),
                        ));
                    });
                }
            }
        }
    }
}

fn set_cursor_locked(locked: bool, mut wins: Query<&mut Window>) {
    let mut win = wins.single_mut();
    win.cursor.grab_mode = if locked {
        let pos = Vec2::new(win.width() / 2., win.height() / 2.);
        win.set_cursor_position(Some(pos));
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };

    win.cursor.visible = !locked;
}

#[derive(Event)]
pub struct WinClicked;

fn win_clicked(
    mut selected: EventWriter<WinClicked>,
    wins: Query<&Window>,
    mouse: Res<Input<MouseButton>>,
) {
    if wins.single().cursor_position().is_some() && mouse.get_just_pressed().len() > 0 {
        selected.send(WinClicked);
    }
}

fn lock_cursor(wins: Query<&mut Window>) {
    set_cursor_locked(true, wins);
}

fn unlock_cursor(wins: Query<&mut Window>) {
    set_cursor_locked(false, wins);
}

#[derive(Component, Default)]
pub struct LookRotation {
    pub pitch: f32,
    pub yaw: f32,
}

fn look(
    mut motions: EventReader<MouseMotion>,
    mut players: Query<(&mut LookRotation, &mut Transform), With<Camera3d>>,
    wins: Query<&Window>,
) {
    const MOUSE_SENSITIVITY: f32 = 0.000_002;

    let win = wins.single();

    if let CursorGrabMode::None = win.cursor.grab_mode {
        motions.clear();
        return;
    }

    let win_scale = win.height().min(win.width());
    let Ok((mut rot, mut tf)) = players.get_single_mut() else {
        return;
    };

    for motion in motions.read() {
        rot.pitch = (rot.pitch - motion.delta.y * win_scale * MOUSE_SENSITIVITY)
            .clamp(-FRAC_PI_2, FRAC_PI_2);
        rot.yaw -= motion.delta.x * win_scale * MOUSE_SENSITIVITY;

        tf.rotation =
            Quat::from_axis_angle(Vec3::Y, rot.yaw) * Quat::from_axis_angle(Vec3::X, rot.pitch);
    }
}

#[derive(Component)]
pub struct ResetVel;

pub fn reset_vel(mut reseters: Query<&mut LinearVelocity, With<ResetVel>>) {
    for mut vel in &mut reseters {
        **vel = Vec3::ZERO;
    }
}

#[derive(Component)]
struct Player;

fn move_player(
    mut players: Query<&mut LinearVelocity, With<Player>>,
    cameras: Query<&LookRotation, With<Camera3d>>,
    keys: Res<Input<ScanCode>>,
) {
    const MOVE_SPEED: f32 = 4.;

    let Ok(mut vel) = players.get_single_mut() else {
        return;
    };

    let input = Vec2::new(
        (keys.pressed(ScanCode(0x20)) as i8 - keys.pressed(ScanCode(0x1E)) as i8) as f32,
        (keys.pressed(ScanCode(0x1F)) as i8 - keys.pressed(ScanCode(0x11)) as i8) as f32,
    );

    **vel += Quat::from_axis_angle(Vec3::Y, cameras.single().yaw)
        .mul_vec3((input.normalize_or_zero() * MOVE_SPEED).extend(0.).xzy());
}
