use bevy::prelude::Component;

#[derive(Component)]
pub struct Dot{
    pub direction: Direction,
    pub direction_x: DirectionX,
}

#[derive(Component)]
pub struct Speed{
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct CollidedWithPlatform(pub bool);
#[derive(Component)]
pub struct MovementState(pub DotState);

#[derive(Component)]
pub struct Platform;

#[derive(Component)]
pub struct Stationary;

#[derive(Component)]
pub struct Movable;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum DirectionX {
    Right,
    Left,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum DotState {
    Jumping,
    Standing,
    Falling,
}
