use std::f32::consts::TAU;

use bevy::{color::palettes::css::MAGENTA, prelude::*};
use bevy_vector_shapes::prelude::*;

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin::default()))
        .add_systems(Startup, init)
        .add_systems(Update, (rotate_camera, draw_shapes))
        .run();
}

const CAMERA_CIRCLE_RADIUS: f32 = 100.;
const CAMERA_HEIGHT: f32 = 700.;
const CAMERA_FOCUS_HEIGHT: f32 = 430.;

fn camera_angle_to_tsf(angle: f32) -> Transform {
    let camera_plane_point = Vec2::from_angle(angle) * CAMERA_CIRCLE_RADIUS;
    Transform::from_xyz(camera_plane_point.x, CAMERA_HEIGHT, camera_plane_point.y)
        .looking_at(Vec3::Y * CAMERA_FOCUS_HEIGHT, Vec3::Y)
}

fn init(mut cmds: Commands) {
    cmds.spawn((
        CamAngle(0.),
        Camera3dBundle {
            transform: camera_angle_to_tsf(0.),
            ..default()
        },
    ));

    for i in 0..16 {
        let plane_point = Vec2::from_angle(i as f32 / 16. * TAU) * 250.;
        cmds.spawn(DrawShape {
            pos: Vec3::new(plane_point.x, 48.5, plane_point.y),
            log: i == 4,
        });
    }
}

#[derive(Component, Deref, DerefMut)]
struct CamAngle(f32);

fn rotate_camera(mut cameras_query: Query<(&mut CamAngle, &mut Transform)>, time: Res<Time>) {
    let (mut angle, mut tsf) = cameras_query.single_mut();
    **angle += time.delta_seconds() * 0.5;
    *tsf = camera_angle_to_tsf(**angle);
}

#[derive(Component)]
struct DrawShape {
    pos: Vec3,
    log: bool,
}

const WIDTH: f32 = 20.;
const HEIGHT: f32 = 2.;
const BORDER: f32 = 1.6;

fn draw_shapes(
    shapes_query: Query<&DrawShape>,
    cameras_query: Query<&GlobalTransform, With<Camera>>,
    mut painter: ShapePainter,
) {
    let Ok(cam_gtsf) = cameras_query.get_single() else {
        return;
    };
    let cam_tsf = cam_gtsf.compute_transform();
    let cam_pos = cam_tsf.translation;
    let cam_up = cam_tsf.up();

    painter.set_3d();

    painter.alignment = Alignment::Billboard;
    for v in [Vec3::X, Vec3::Z] {
        painter.color = MAGENTA.into();
        painter.line(v * 1000. + Vec3::Y * 50., v * -1000. + Vec3::Y * 50.);
    }
    painter.alignment = Alignment::Flat;

    for &DrawShape { pos, log } in &shapes_query {
        painter.color = Color::BLACK;
        // Look away from the camera so positive X is to the right
        painter.transform = Transform::from_translation(pos).looking_to(pos - cam_pos, cam_up);

        // Background
        painter.rect(Vec2::new(WIDTH + BORDER, HEIGHT + BORDER));

        let width = WIDTH;
        let height = HEIGHT;
        let right = width / 2.;
        let left = -right;

        // Health
        painter.color = if log { Srgba::GREEN } else { Srgba::RED }.into();
        painter.translate(Vec3::new(left * 0.5, 0., 1.));
        painter.rect(Vec2::new(0.5 * width, height));

        if log {
            info!(
                "back: {pos}, health: {}, cam: {cam_pos}",
                painter.transform.translation
            );
        }
    }
}
