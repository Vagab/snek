use bevy::prelude::*;

#[derive(Component)]
pub struct Size {
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
