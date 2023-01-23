use crate::Position;
use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct LastTailPosition(pub Option<Position>);
