use crate::prelude::*;

use self::template::Templates;

pub mod template;

const TEMPLATES_PATH: &str = "resources/template.ron";

const PLAYER_HP: i32 = 10;
const PLAYER_VIEW_RADIUS: i32 = 8;
const PLAYER_GLYPH: char = '@';

const AMULET_OF_YALA_GLYPH: char = '|'; 

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player{floor_number: 0},
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(PLAYER_GLYPH),
            },
            Health{current: PLAYER_HP, max: PLAYER_HP},
            FieldOfView::new(PLAYER_VIEW_RADIUS),
        )
    );
}

pub fn spawn_amulet_of_yala(ecs: &mut World, position: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        position,
        Render{
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437(AMULET_OF_YALA_GLYPH),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

pub fn spawn_floor(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let templates = Templates::load(TEMPLATES_PATH);
    templates.spawn_entities(
        ecs,
        rng,
        level,
        spawn_points,
    );
}
