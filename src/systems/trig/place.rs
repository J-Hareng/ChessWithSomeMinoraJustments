use bevy::prelude::*;

#[derive(Event)]
struct pawn_selected {}

struct place_pawn {
    pawn: crate::comp::pawn::Pawn,
    tile: crate::comp::tile::Tile,
}
