#[allow(unused_variables)]
use bevy::prelude::*;
use bevy::{
    ecs::component,
    math::{vec2, vec3},
};

#[derive(Component)]
struct Tile {
    color: Color,
    loc: Vec2,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Schachbrett erstellen
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    //     Transform::from_xyz(0.0, 0.5, 0.0),
    // ));

    //  camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
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

            println!("Color: {:?}", color);

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
                Tile {
                    color,
                    loc: vec2(x as f32, z as f32),
                },
            ));
        }
    }
}
