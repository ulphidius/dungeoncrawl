mod map_render;
mod player_inputs;
mod entity_render;
mod collisions;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    return Schedule::builder()
        .add_system(player_inputs::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(collisions::collisions_system())
        .build();
}
