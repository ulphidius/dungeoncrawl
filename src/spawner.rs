use crate::prelude::*;

const PLAYER_HP: i32 = 10;
const PLAYER_VIEW_RADIUS: i32 = 8;

const MONSTER_VIEW_RADIUS: i32 = 6;

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
            FieldOfView::new(PLAYER_VIEW_RADIUS),
        )
    );
}

pub fn spawn_entity(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator, 
    pos: Point,
) {
    return match rng.range(0, 100) {
        0..=85 => spawn_monster(ecs, rng, pos),
        86..=95 => spawn_healing_potion(ecs, pos),
        _ => spawn_magic_mapper(ecs, pos),
    };
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
            Name(name),
            FieldOfView::new(MONSTER_VIEW_RADIUS),
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

pub fn spawn_amulet_of_yala(ecs: &mut World, position: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        position,
        Render{
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

pub fn spawn_healing_potion(ecs: &mut World, position: Point) {
    ecs.push(
        (
            Item,
            position,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('!')
            },
            Name("Healing Potion".to_string()),
            ProvidesHealing{amount: 6},
        )

    );
}

pub fn spawn_magic_mapper(ecs: &mut World, position: Point) {
    ecs.push((
        Item,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('{')
        },
        Name("Dungeon Map".to_string()),
        ProvidesDungeonMap
    ));
}
