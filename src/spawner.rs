use crate::prelude::*;

const PLAYER_HP: i32 = 10;

const GOBLIN_HEALTH: i32 = 1;
const GOBLIN_NAME: &str = "Goblin";

const ORC_HEALTH: i32 = 2;
const ORC_NAME: &str = "Orc";

const OGRE_HEALTH: i32 = 3;
const OGRE_NAME: &str = "Ogre";

const ETTIN_HEALTH: i32 = 4;
const ETTIN_NAME: &str = "Ettin";

type Monster = (i32, String, FontCharType);

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            },
            Health{current: PLAYER_HP, max: PLAYER_HP},
        )
    );
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator, 
    pos: Point,
) {
    let (health, name, glypth) = random_monster(rng);

    ecs.push(
        (
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: glypth,
            },
            ChasingPlayer{},
            Health{ current: health, max: health },
            Name(name)
        )
    );
}

fn random_monster(rng: &mut RandomNumberGenerator) -> Monster {
    return match rng.range(0, 100) {
        0..=50 => goblin(),
        51..=80 => orc(),
        81..=95 => ogre(),
        _ => ettin(),
    };
}

fn goblin() -> Monster {
    return (GOBLIN_HEALTH, GOBLIN_NAME.to_string(), to_cp437('g'));
}

fn orc() -> Monster {
    return (ORC_HEALTH, ORC_NAME.to_string(), to_cp437('o'));
}

fn ogre() -> Monster {
    return (OGRE_HEALTH, OGRE_NAME.to_string(), to_cp437('O'));
}

fn ettin() -> Monster {
    return (ETTIN_HEALTH, ETTIN_NAME.to_string(), to_cp437('E'));
}
