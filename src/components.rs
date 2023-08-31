use bevy::prelude::Component;

#[derive(Component)]
pub struct Dot {
    pub direction: Direction,
    pub direction_x: DirectionX,
}

#[derive(Component)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct CollidedWithPlatform(pub bool);

#[derive(Component)]
pub struct YMovementState(pub YDotState);

#[derive(Component)]
pub struct XMovementState(pub XDotState);

#[derive(Component)]
pub struct JumpingState(pub JumpState);

#[derive(Component)]
pub struct Platform;

#[derive(Component)]
pub struct Stationary;

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Camera;

#[derive(Component)]
pub struct Coin;

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
pub enum YDotState {
    Jumping,
    Standing,
    Falling,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum XDotState {
    Decelerating,
    Stopped,
    Accelerating,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub enum JumpState {
    NoJump,
    SingleJump,
    DoubleJump,
}
