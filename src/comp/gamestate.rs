use ::bevy::prelude::*;

#[derive(Component)]
pub struct gamestate {
    pub Selected_pawn: Option<crate::mem::val::gridLocation>,
}
impl gamestate {
    pub fn new() -> Self {
        gamestate {
            Selected_pawn: None,
        }
    }
}
