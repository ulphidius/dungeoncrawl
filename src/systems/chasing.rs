use crate::prelude::*;

const MAX_DEPTH: f32 = 1024.0;
const ATTACK_DISTANCE: f32 = 1.2; // Floating number are imprecise so we a value bigger than 1 and lower than 1.5 to avoid diagonal move

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(
    #[resource] map: &Map,
    ecs: &SubWorld,
    command: &mut CommandBuffer,
) {
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_position = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = map_idx(player_position.x, player_position.y);

    let search_target = vec![player_idx];
    let dijktra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_target,
        map,
        MAX_DEPTH,
    );

    movers.iter(ecs)
        .for_each(|(entity, position, _)| {
            let idx = map_idx(position.x, position.y);
            if let Some(destination) = DijkstraMap::find_lowest_exit(
                &dijktra_map,
                idx,
                map,
            ) {
                let player_distance = DistanceAlg::Pythagoras.distance2d(*position, *player_position);
                let destination = if player_distance > ATTACK_DISTANCE {
                    map.index_to_point2d(destination)
                } else {
                    *player_position
                };

                let mut attacked = false;
                positions.iter(ecs)
                    .filter(|(_, target_position, _)| **target_position == destination)
                    .for_each(|(victim, _, _)| {
                        if ecs.entry_ref(*victim).unwrap().get_component::<Player>().is_ok() {
                            command.push(((), WantsToAttack{
                                attacker: *entity,
                                victim: *victim,
                            }));
                        }
                        attacked = true;
                    });

                if !attacked {
                    command.push(((), WantsToMove{
                        entity: *entity,
                        destination
                    }));
                }
            }
        });
}
