use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState,
    #[resource] map: &Map,
) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let current_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state, 
    };

    let amulet_default_position = Point::new(-1, -1);
    let amulet_position = amulet.iter(ecs)
        .nth(0)
        .unwrap_or(&amulet_default_position);

    player_hp.iter(ecs)
        .for_each(|(hp, position)| {
            if hp.current < 1 {
                new_state = TurnState::GameOver;
            }
            if position == amulet_position {
                new_state = TurnState::Victory;
            }

            let idx = map.point2d_to_index(*position);
            if map.tiles[idx] == TileType::Exit {
                new_state = TurnState::NewFloor;
            }
        });
    *turn_state = new_state;
}
