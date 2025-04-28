use crate::comp::tile::*;
use crate::mem;
use bevy::math::{vec2, vec3};
#[allow(unused_variables)]
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
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

    //  camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        SceneRoot(
            asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("models/SiehtSchonAusWieEinButtplug.gltf"),
            ),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 0, 255))),
    ));

    commands.spawn(crate::comp::gamestate::gamestate::new());
}
