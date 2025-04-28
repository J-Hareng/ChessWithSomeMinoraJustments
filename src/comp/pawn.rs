use crate::{mem, systems};
use bevy::ecs::query;
use bevy::math::{vec2, vec3};
use bevy::{prelude::*, transform};

const PAWN_RADIUS: f32 = 0.2;
const PAWN_HEIGHT: f32 = 0.7;
const PAWN_Y_OFFSET: f32 = PAWN_HEIGHT * 0.5;

const PAWN_COLOR: Color = Color::rgb(1.0, 0.1, 0.8);
const PAWN_COLOR_HOVER: Color = Color::rgb(0.2, 0.1, 0.8);

#[derive(Component)]
pub struct Pawn {
    pev: i8,
    position: mem::val::gridLocation,
    color: Color,
    pub selected: bool,
}

impl Pawn {
    pub fn new(pev: i8, position: mem::val::gridLocation, color: Color) -> Self {
        Pawn {
            pev,
            position,
            color,
            selected: false,
        }
    }
}

pub fn spawn_pawn(
    query: Query<(&crate::comp::tile::Tile, &Transform)>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (tile, transform) in query.iter() {
        let tile_loc = &tile.grid_cords;
        println!("Tile: {:?}", tile_loc);

        if tile_loc.y == 6 && tile_loc.x == 1 {
            commands
                .spawn((
                    Mesh3d(meshes.add(Cylinder::new(PAWN_RADIUS, PAWN_HEIGHT))),
                    Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y + PAWN_Y_OFFSET,
                        transform.translation.z,
                    ),
                    MeshMaterial3d(materials.add(PAWN_COLOR)),
                    Pawn::new(1, tile_loc.clone(), PAWN_COLOR),
                ))
                // Change Color
                .observe(update_material_on::<Pointer<Over>>(
                    materials.add(PAWN_COLOR_HOVER.clone()),
                ))
                .observe(update_material_on::<Pointer<Out>>(
                    materials.add(PAWN_COLOR.clone()),
                ))
                // Drag and Drop
                .observe(select_pawn);
        }
    }
}

fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.entity()) {
            material.0 = new_material.clone();
        }
    }
}

fn select_pawn(
    t: Trigger<Pointer<Down>>,
    mut query: Query<&mut Transform>,
    mut pawn_query: Query<&mut Pawn>,
    mut gamestate_query: Query<&mut crate::comp::gamestate::gamestate>,
) {
    let mut transform = query.get_mut(t.entity()).unwrap();

    let mut pawn = pawn_query.get_mut(t.entity()).unwrap();

    let mut gamestate = gamestate_query.get_single_mut().unwrap();

    if gamestate.Selected_pawn.is_none() {
        pawn.selected = true;
        gamestate.Selected_pawn = Some(pawn.position.clone());
    } else if gamestate.Selected_pawn == Some(pawn.position) {
        pawn.selected = false;
        gamestate.Selected_pawn = None;
    } else {
        return;
    }

    if transform.translation.y >= PAWN_Y_OFFSET + 0.1 {
        transform.translation.y = PAWN_Y_OFFSET;
    } else {
        transform.translation.y = 0.5
    }
    // Do something with the selected pawn
}
