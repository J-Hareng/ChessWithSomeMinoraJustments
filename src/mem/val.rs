use bevy::prelude::*;
#[derive(Clone)] //
pub struct gridLocation {
    pub x: i8,
    pub y: i8,
}

impl gridLocation {
    pub fn new(x: i8, y: i8) -> Self {
        gridLocation { x, y }
    }
    pub fn from_vec2(v: Vec2) -> Self {
        gridLocation {
            x: v.x as i8,
            y: v.y as i8,
        }
    }
}
impl std::fmt::Display for gridLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl std::fmt::Debug for gridLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
