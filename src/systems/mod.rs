mod map_render;
mod player_inputs;
mod entity_render;
mod random_move;
mod end_turn;
mod movement;
mod hud;
mod tooltips;
mod combat;
mod chasing;
mod field_of_view;

use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    return Schedule::builder()
        .add_system(player_inputs::player_input_system())
        .add_system(field_of_view::field_of_view_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build();
}

pub fn build_player_scheduler() -> Schedule {
    return Schedule::builder()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(field_of_view::field_of_view_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build();
}

pub fn build_monster_scheduler() -> Schedule {
    return Schedule::builder()
        .add_system(random_move::random_move_system())
        .add_system(chasing::chasing_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(field_of_view::field_of_view_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build();
}
