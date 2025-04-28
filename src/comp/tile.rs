use crate::mem;
#[allow(unused_variables)]
use bevy::prelude::*;
use bevy::{
    asset::UnknownTyped,
    math::{vec2, vec3},
    text::cosmic_text::rustybuzz::script::UNKNOWN,
};

#[derive(Component, Clone)]
pub struct Tile {
    color: Color,
    pub grid_cords: mem::val::gridLocation,
}

impl Tile {
    pub fn new(color: Color, grid_cords: mem::val::gridLocation) -> Self {
        Tile { color, grid_cords }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}
