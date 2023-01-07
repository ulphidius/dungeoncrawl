use crate::prelude::*;
use super::MapArchitect;

pub struct EmptyArchitect{}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, map_size: usize, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut map_builder = MapBuilder {
            map: Map::new(map_size),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: super::themes::DungeonTheme::new()
        };
        map_builder.fill(TileType::Floor);
        map_builder.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        map_builder.amulet_start = map_builder.find_most_distant();
        for _ in 0..50 {
            map_builder.monster_spawns.push(
                Point::new(
                    rng.range(1, SCREEN_WIDTH),
                    rng.range(1, SCREEN_WIDTH)
                )
            );
        }

        return map_builder;
    }
}
