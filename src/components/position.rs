use bevy::prelude::*;

#[derive(Component, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32,
}
