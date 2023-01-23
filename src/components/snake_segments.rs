use bevy::prelude::*;

pub const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Default, Deref, DerefMut, Resource)]
pub struct SnakeSegments(pub Vec<Entity>);
