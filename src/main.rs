use bevy::prelude::*;
use seldom_state::prelude::*;

#[derive(Clone, Component)]
pub struct IdleState;

pub struct HitEvent {
    hit_entity: Entity,
}

pub struct HurtByPlayer;

impl BoolTrigger for HurtByPlayer {
    type Param<'w, 's> = EventReader<'w, 's, HitEvent>;

    fn trigger(&self, entity: Entity, mut hit_events: Self::Param<'_, '_>) -> bool {
        for hit in hit_events.iter() {
            if hit.hit_entity == entity {
                return true;
            }
        }

        false
    }
}

#[derive(Clone, Component)]
pub struct FollowState {
    pub target: Entity,
}

#[derive(Clone, Component)]
pub struct AttackState {
    countdown: f32,
    pub target: Entity,
}

pub struct AttackDistance {
    pub target: Entity,
    pub range: f32,
}

impl Trigger for AttackDistance {
    type Param<'w, 's> = (
        Query<'w, 's, (&'static Transform, Option<&'static AttackState>)>,
        Res<'w, Time>,
    );
    type Ok = f32;
    type Err = f32;

    // Return `Ok` to trigger and `Err` to not trigger
    fn trigger(
        &self,
        entity: Entity,
        (transforms, _time): Self::Param<'_, '_>,
    ) -> Result<f32, f32> {
        if let Some(attack) = transforms.get(entity).unwrap().1 {
            if attack.countdown <= 0. {
                return Ok(0.);
            }
        }
        let delta = transforms
            .get(self.target)
            .unwrap()
            .0
            .translation
            .truncate()
            - transforms.get(entity).unwrap().0.translation.truncate();

        let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();
        (distance <= self.range).then_some(distance).ok_or(distance)
    }
}

pub struct LineOfSight {
    pub target: Entity,
    pub range: f32,
}

impl Trigger for LineOfSight {
    type Param<'w, 's> = (Query<'w, 's, &'static Transform>, Res<'w, Time>);
    type Ok = f32;
    type Err = f32;

    // Return `Ok` to trigger and `Err` to not trigger
    fn trigger(
        &self,
        entity: Entity,
        (transforms, _time): Self::Param<'_, '_>,
    ) -> Result<f32, f32> {
        let delta = transforms.get(self.target).unwrap().translation.truncate()
            - transforms.get(entity).unwrap().translation.truncate();

        let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();
        (distance <= self.range).then_some(distance).ok_or(distance)
    }
}

#[derive(Component)]
struct Player {
    target: Entity,
}

#[derive(Component)]
struct Enemy;

fn init(mut commands: Commands) {
    let player = commands.spawn(Transform::default()).id();

    let mut spawn_enemy = || {
        commands
            .spawn((
                StateMachine::default()
                    .set_trans_logging(true)
                    .trans::<IdleState>(HurtByPlayer, FollowState { target: player })
                    .trans::<FollowState>(
                        AttackDistance {
                            target: player,
                            range: 50.,
                        },
                        AttackState {
                            target: player,
                            countdown: 1.6,
                        },
                    )
                    .trans::<FollowState>(
                        Trigger::not(LineOfSight {
                            target: player,
                            range: 130.,
                        }),
                        IdleState,
                    )
                    .trans::<AttackState>(
                        Trigger::not(AttackDistance {
                            target: player,
                            range: 20.,
                        }),
                        FollowState { target: player },
                    ),
                IdleState,
                Transform::from_translation(Vec3::X * 100.),
                Enemy,
            ))
            .id()
    };

    let enemy = spawn_enemy();
    spawn_enemy();

    commands.entity(player).insert(Player { target: enemy });
}

fn move_player(
    mut players: Query<&mut Transform, With<Player>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    players.single_mut().translation.x += time.delta_seconds()
        * 10.
        * (keys.pressed(KeyCode::Right) as i32 - keys.pressed(KeyCode::Left) as i32) as f32;
}

fn attack(
    mut hit_events: EventWriter<HitEvent>,
    players: Query<&Player>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        hit_events.send(HitEvent {
            hit_entity: players.single().target,
        });
    }
}

fn log(players: Query<(&Transform, &Player)>, enemies: Query<&Transform, With<Enemy>>) {
    let (tf, player) = players.single();
    let player_pos = tf.translation.x;
    let enemy_pos = enemies.get(player.target).unwrap().translation.x;
    info!("Player: {}, Enemy: {}", player_pos, enemy_pos);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StateMachinePlugin)
        .add_event::<HitEvent>()
        .insert_resource(FixedTime::new_from_secs(0.5))
        .add_startup_system(init)
        .add_systems((
            move_player,
            attack,
            log.in_schedule(CoreSchedule::FixedUpdate),
        ))
        .run();
}
