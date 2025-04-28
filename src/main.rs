#[allow(unused_mut)]
use ::bevy::prelude::*;
use bevy::{
    color::palettes::tailwind::*, ecs::component, picking::pointer::PointerInteraction, prelude::*,
};
use bevy_flycam::prelude::*;

mod comp;
mod mem;
mod systems;
fn main() {
    App::new()
        // .add_plugins(NoCameraPlayerPlugin)
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, systems::setup::setup)
        .add_systems(Startup, comp::tile::spawn_chessboard)
        .add_systems(PostStartup, comp::pawn::spawn_pawn)
        // debug camera
        .add_plugins(PlayerPlugin)
        .run();
}

// /// set up a simple 3D scene
// fn setup(
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // circular base

//     // cube
// commands.spawn((
//     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
//     MeshMaterial3d(materials.add(Color::srgb_u8(124, 0, 255))),
//     Transform::from_xyz(0.0, 0.5, 0.0),
// ));
//     // light
//     commands.spawn((
//         PointLight {
//             shadows_enabled: true,
//             color: Color::srgb_u8(124, 0, 255),
//             ..default()
//         },
//         Transform::from_xyz(4.0, 8.0, 4.0),
//     ));
//     // camera
//     commands.spawn((
//         Camera3d::default(),
//         Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
//     ));
// }
