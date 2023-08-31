use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::colors::{BLUE, ORANGE};
use crate::components::{Camera, CollidedWithPlatform, DirectionX, Dot, Movable, MovementState, JumpingState, Platform, Speed, Stationary};
use crate::components::Direction::{Down, Up};
use crate::components::DirectionX::{Left, Right};
use crate::components::DotState::{Falling, Jumping, Standing};
use crate::components::JumpState::{NoJump, SingleJump, DoubleJump};

mod colors;
mod components;

fn main() {
    App::new()
        .insert_resource(ClearColor(BLUE))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_gravity.before(apply_collision), apply_collision, death_dot))
        .add_systems(PostUpdate, (move_dot, handle_keyboard, camera_follow_dot.after(move_dot)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), Camera));

    // Circle
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(10.).into()).into(),
        material: materials.add(ColorMaterial::from(ORANGE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    }, Movable, CollidedWithPlatform(false), MovementState(Falling), JumpingState(NoJump),
                    Speed { x: 0., y: 0. }, Dot { direction: Down, direction_x: DirectionX::Right }));


    let map = include_str!("./map.txt");
    init_map(map, commands);
}

fn init_map(
    map: &str,
    mut commands: Commands,
) {
    let mut x = 0;
    let mut y = 0;

    map.chars().for_each(|c| {
        if c == '-' {
            x += 1;
            commands.spawn((SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-700. + (1. * x as f32 * 10.), 100. - 25. * y as f32, -1.)),
                ..default()
            }, Platform, Stationary));
        }
        if c == ' ' {
            x += 1;
        };
        if c == '\n' {
            y += 1;
            x = 0;
        }
    });
}

fn apply_gravity(
    mut entities_with_speed: Query<(&mut Speed, &CollidedWithPlatform, &mut MovementState), (With<Speed>, Without<Stationary>)>,
    time: Res<Time>) {
    for (mut speed, collided_with_platform, mut movable_state) in entities_with_speed.iter_mut() {
        if collided_with_platform.0 == false {
            speed.y -= 9.8 * time.delta().as_secs_f32();
            speed.y = speed.y.clamp(-3., 3.);
            if speed.y < 0. {
                movable_state.0 = Falling;
            } else {
                movable_state.0 = Jumping;
            };
        }
    }
}

fn camera_follow_dot(
    mut dot_transform_query: Query<( &Transform), (With<Dot>, Without<Camera>)>,
    mut camera_query: Query<(&mut Transform), (With<Camera>, Without<Dot>)>,
    time: Res<Time>) {
    for (mut dot_transform) in dot_transform_query.iter_mut() {
        let delta_y = dot_transform.translation.y - camera_query.single_mut().translation.y;
        let delta_x = dot_transform.translation.x - camera_query.single_mut().translation.x;
        if (delta_y.abs() > 10.) {
            camera_query.single_mut().translation.y += 1. * time.delta().as_secs_f32() * delta_y;
        }
        if (delta_x.abs() > 10.) {
            camera_query.single_mut().translation.x += 1. * time.delta().as_secs_f32() * delta_x;
        }
    }
}

fn death_dot(
    mut commands: Commands,
    mut dot_transform_query: Query<(&Transform, Entity), (With<Dot>)>,
    mut all_entities: Query<(  Entity), (With<Platform>)>,
) {
    dot_transform_query.iter_mut().for_each(|(transform, entity)| {
        if transform.translation.y < -1000. {
            commands.entity(entity).despawn();

            all_entities.iter().for_each(|entity| {
                commands.entity(entity).despawn();
            })
        }
    });
}

fn apply_collision(mut movable_query: Query<(&mut Transform, &mut CollidedWithPlatform, &mut Speed, &mut MovementState, &mut JumpingState), (With<Movable>, Without<Platform>)>,
                   platform_query: Query<(&Transform), (With<Platform>, Without<Movable>)>) {
    let x = for (mut transform, mut collided_with_platform, mut speed, mut movable_state, mut jumping_state) in movable_query.iter_mut() {
        let prev = collided_with_platform.0;
        let mut y = 0.;
        collided_with_platform.0 = platform_query.iter().any(|platform_transform| {
            let collision = speed.y < 0. && transform.translation.abs_diff_eq(platform_transform.translation, 10.);
            if collision {
                y = platform_transform.translation.y
            }
            collision
        });

        if collided_with_platform.0 && prev != collided_with_platform.0 {
            transform.translation.y = y + 10.;
            speed.y = 0.;
            movable_state.0 = Standing;
            jumping_state.0 = NoJump;
        };
    };
    x
}

fn move_dot(mut dot_query: Query<(&mut Transform, &mut Dot, &mut Speed, &CollidedWithPlatform), (Without<Platform>, With<Dot>)>,
            time: Res<Time>) {
    let time_delta = time.delta().as_secs_f32();
    for (mut dot_transform, dot, speed, collided_with_platform) in dot_query.iter_mut() {
        if !collided_with_platform.0 {
            dot_transform.translation.y += speed.y * 3. * time_delta * 100.;
        }

        match dot.direction_x {
            Right => {
                dot_transform.translation.x += time_delta * speed.x * 200.;
            }
            Left => {
                dot_transform.translation.x -= time_delta * speed.x * 200.;
            }
        }
    }
}

fn handle_keyboard(keyboard_input: Res<Input<KeyCode>>, mut dot_query: Query<(&mut Speed, &mut Dot, &mut MovementState, &mut JumpingState), With<Dot>>) {
    for (mut speed, mut dot, mut dot_state, mut jumping_state) in dot_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            dot.direction_x = Left;
            speed.x = 2.;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            dot.direction_x = Right;
            speed.x = 2.;
        }

        if (keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up)
            || keyboard_input.just_pressed(KeyCode::Space)) && (dot_state.0 == Standing || jumping_state.0 != DoubleJump) {
            dot.direction = Up;
            speed.y = 3.;

            // Only allow one jump when falling from platform
            if dot_state.0 == Falling && jumping_state.0 == NoJump {
                jumping_state.0 = DoubleJump;
                return;
            }

            dot_state.0 = Jumping;

            if jumping_state.0 == NoJump {
                jumping_state.0 = SingleJump
            } else if jumping_state.0 == SingleJump {
                jumping_state.0 = DoubleJump;
            }
        }
        if keyboard_input.just_released(KeyCode::A) || keyboard_input.just_released(KeyCode::Left)
            || keyboard_input.just_released(KeyCode::D) || keyboard_input.just_released(KeyCode::Right) {
            speed.x = 0.
        }
    }
}
