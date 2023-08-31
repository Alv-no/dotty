use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::colors::{BLUE, ORANGE};
use crate::components::{CollidedWithPlatform, DirectionX, Dot, Movable, MovementState, Platform, Speed, Stationary};
use crate::components::Direction::{Down, Up};
use crate::components::DirectionX::{Left, Right};
use crate::components::DotState::{Falling, Jumping, Standing};

mod colors;
mod components;

fn main() {
    App::new()
        .insert_resource(ClearColor(BLUE))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_gravity.before(apply_collision), apply_collision))
        .add_systems(PostUpdate, (move_dot, handle_keyboard))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let map = include_str!("./map.txt");

    // Circle
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(10.).into()).into(),
        material: materials.add(ColorMaterial::from(ORANGE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    }, Movable, CollidedWithPlatform(false), MovementState(Falling),
                    Speed { x: 0., y: 0. }, Dot { direction: Down, direction_x: DirectionX::Right }));


    let mut x = 0;
    let mut y = 0;

    map.chars().for_each(|c| {
        if c == '─' {
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

fn apply_collision(mut movable_query: Query<(&mut Transform, &mut CollidedWithPlatform, &mut Speed, &mut MovementState), (With<Movable>, Without<Platform>)>,
                   platform_query: Query<(&Transform), (With<Platform>, Without<Movable>)>) {
    let x = for (mut transform, mut collided_with_platform, mut speed, mut movable_state) in movable_query.iter_mut() {
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
        };
    };
    x
}

fn move_dot(mut dot_query: Query<(&mut Transform, &mut Dot, &mut Speed, &CollidedWithPlatform), (Without<Platform>, With<Dot>)>,
            time: Res<Time>) {
    for (mut dot_transform, dot, speed, collided_with_platform) in dot_query.iter_mut() {
        if !collided_with_platform.0 {
            dot_transform.translation.y += speed.y * 3.;
        }

        match dot.direction_x {
            Right => {
                dot_transform.translation.x += time.delta().as_secs_f32() * speed.x * 200.;
            }
            Left => {
                dot_transform.translation.x -= time.delta().as_secs_f32() * speed.x * 200.;
            }
        }
    }
}

fn handle_keyboard(keyboard_input: Res<Input<KeyCode>>, mut dot_query: Query<(&mut Speed, &mut Dot, &mut MovementState), With<Dot>>) {
    for (mut speed, mut dot, mut dot_state) in dot_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            dot.direction_x = Left;
            speed.x = 2.;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            dot.direction_x = Right;
            speed.x = 2.;
        }

        if (keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up)
            || keyboard_input.just_pressed(KeyCode::Space)) && dot_state.0 == Standing {
            dot.direction = Up;
            dot_state.0 = Jumping;
            speed.y = 3.;
        }
        if keyboard_input.just_released(KeyCode::A) || keyboard_input.just_released(KeyCode::Left)
            || keyboard_input.just_released(KeyCode::D) || keyboard_input.just_released(KeyCode::Right) {
            speed.x = 0.
        }
    }
}
