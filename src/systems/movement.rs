use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_in_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(field_of_view) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, field_of_view.clone_dirty());
                
                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    field_of_view.visible_tiles.iter()
                        .for_each(|position| {
                            map.revealed_tiles[map_idx(position.x, position.y)] = true;
                        })    
                }
            }
        }
    }

    commands.remove(*entity);
}
