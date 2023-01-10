use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
pub fn player_input(
    ecs: &mut SubWorld,
    command: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query()
        .filter(component::<Enemy>());

    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::G => {
                let (player, player_position) = players.iter(ecs)
                    .find_map(|(entity, position)| Some((*entity, *position)))
                    .unwrap();
                let mut items = <(Entity, &Item, &Point)>::query();
                items.iter(ecs)
                    .filter(|(_entity, _item, &item_position)| item_position == player_position)
                    .for_each(|(entity, _item, _item_position)| {
                        command.remove_component::<Point>(*entity);
                        command.add_component(*entity, Carried(player));
                    });
                Point::new(0, 0)
            },
            VirtualKeyCode::Key1 => use_item(0, ecs, command),
            VirtualKeyCode::Key2 => use_item(1, ecs, command),
            VirtualKeyCode::Key3 => use_item(2, ecs, command),
            VirtualKeyCode::Key4 => use_item(3, ecs, command),
            VirtualKeyCode::Key5 => use_item(4, ecs, command),
            VirtualKeyCode::Key6 => use_item(5, ecs, command),
            VirtualKeyCode::Key7 => use_item(6, ecs, command),
            VirtualKeyCode::Key8 => use_item(7, ecs, command),
            VirtualKeyCode::Key9 => use_item(8, ecs, command),
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players.iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            
            enemies.iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;

                    command.push(((), WantsToAttack{
                        attacker: player_entity,
                        victim: *entity,
                    }));
                });

            if !hit_something {
                command.push(((), WantsToMove{
                    entity: player_entity,
                    destination
                }));
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}

fn use_item(n: usize, ecs: &mut SubWorld, command: &mut CommandBuffer) -> Point {
    let player_entity = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap();
    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, (_, _, _))| *item_count == n)
        .find_map(|(_, (item_entity, _, _))| Some(*item_entity));

    if let Some(item_entity) = item_entity {
        command.push(
            ((),
            ActivateItem {
                used_by: player_entity,
                item: item_entity,
            })
        );
    }

    return Point::zero();
}
