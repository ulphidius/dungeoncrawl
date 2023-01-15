use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(WantsToAttack)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity, Entity)> = attackers.iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();
    
    victims.iter().for_each(|(message, attacker, victim)| {
        let is_player = ecs.entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let damage = compute_damage(ecs, attacker);
        if let Ok(mut health) = ecs.entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>() {

                
                health.current -= damage;
                if health.current < 1 && !is_player {
                    commands.remove(*victim);
                }
        }
        commands.remove(*message);
    });
}

fn compute_damage(ecs: &mut SubWorld, attacker: &Entity) -> i32 {
    let base_attack = match ecs.entry_ref(*attacker) {
        Ok(value) => match value.get_component::<Damage>() {
            Ok(damage) => damage.0.clone(),
            Err(_) => 0,
        },
        Err(_) => 0,
    };

    let weapon_damage: i32 = <(&Carried, &Damage)>::query().iter(ecs)
        .filter(|(carried, _)| carried.0 == *attacker)
        .map(|(_, damage)| damage.0)
        .sum();

    return base_attack + weapon_damage;
}

