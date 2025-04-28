use crate::mem;
#[allow(unused_variables)]
use bevy::prelude::*;
use bevy::{
    asset::UnknownTyped,
    math::{vec2, vec3},
    text::cosmic_text::rustybuzz::script::UNKNOWN,
};

use super::pawn::Pawn;

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

pub fn spawn_chessboard(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile_size = 1.0;
    let board_size = 8;
    let gap = 10.0;

    for x in 0..board_size {
        for z in 0..board_size {
            let is_white = (x + z) % 2 == 0;

            let color = if is_white {
                Color::srgb(1.0, 1.0, 1.0) // Weiß
            } else {
                Color::srgb(0.0, 0.0, 0.0) // Schwarz
            };

            commands.spawn((
                Mesh3d(meshes.add(Mesh::from(Plane3d::new(
                    vec3(0.0, 1.0, 0.0),
                    vec2(tile_size * 0.4, tile_size * 0.4),
                )))),
                MeshMaterial3d(materials.add(color)),
                Transform::from_xyz(
                    (x as f32) - (board_size as f32 / 2.0) + 0.5,
                    0.0,
                    z as f32 - (board_size as f32 / 2.0) + 0.5,
                ),
                Tile::new(color, mem::val::gridLocation::new(x as i8, z as i8)),
            ));
            // .observe(update_tile_materialOnSelection);
        }
    }
}

// fn update_tile_materialOnSelection(
//     t: Trigger<Pointer<Over>>,
//     selected_pawn: Query<&mut Pawn, With<selected>>,
// ) {
// }
