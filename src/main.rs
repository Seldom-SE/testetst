use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin::default()))
        .add_systems(Startup, init)
        .add_systems(Update, draw_shapes)
        .run();
}

const CAM_POS: Vec3 = Vec3::new(0., 70., -10.);
const BACKGROUND_POS: Vec3 = Vec3::new(0., 0., 10.);

fn init(mut cmds: Commands) {
    cmds.spawn(Camera3dBundle {
        transform: Transform::from_translation(CAM_POS).looking_at(BACKGROUND_POS, Dir3::Y),
        ..default()
    });
}

fn draw_shapes(mut painter: ShapePainter) {
    painter.set_3d();

    painter.color = Color::BLACK;
    // Look away from the camera so positive X is to the right
    painter.transform =
        Transform::from_translation(BACKGROUND_POS).looking_to(BACKGROUND_POS - CAM_POS, Dir3::Y);

    let width = 10.;
    let height = 1.;

    // Background
    painter.rect(Vec2::new(width, height));

    // Health
    painter.color = Srgba::RED.into();
    painter.set_translation((CAM_POS + BACKGROUND_POS) / 2.);
    painter.rect(Vec2::new(width, height));
}
