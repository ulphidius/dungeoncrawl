use crate::prelude::*;
use super::MapArchitect;

const STAGGER_DISTANCE: usize = 400;
const GLOBAL_NUMBER_OF_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = GLOBAL_NUMBER_OF_TILES / 3;

pub struct DrunkardWalkArchitect {}

impl MapArchitect for DrunkardWalkArchitect {
    fn new(&mut self, map_size: usize, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut map_builder = MapBuilder {
            map: Map::new(map_size),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        map_builder.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(&center, rng, &mut map_builder.map);

        while map_builder.map.tiles.iter()
            .filter(|tile| **tile == TileType::Floor )
            .count() < DESIRED_FLOOR {
                self.drunkard(
                    &Point::new(
                        rng.range(0, SCREEN_WIDTH),
                        rng.range(0, SCREEN_HEIGHT),
                    ),
                    rng,
                    &mut map_builder.map,
                );

                let dijkstra_map = DijkstraMap::new(
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                    &vec![map_builder.map.point2d_to_index(center)],
                    &map_builder.map,
                    MAX_DIJKSTRA_DEPTH,
                );
                dijkstra_map.map.iter()
                    .enumerate()
                    .filter(|(_, distance)| *distance > &2000.0)
                    .for_each(|(idx, _)| map_builder.map.tiles[idx] = TileType::Wall);
        }

        map_builder.monster_spawns = map_builder.spawn_monster(&center, NUMBER_OF_MONSTER, rng);
        map_builder.player_start = center;
        map_builder.amulet_start = map_builder.find_most_distant();

        return map_builder;
    }
}

impl DrunkardWalkArchitect {
    fn drunkard(
        &mut self,
        start: &Point,
        rng: &mut RandomNumberGenerator,
        map: &mut Map,
    ) {
        let mut drunkard_position = start.clone();
        let mut distance_staggered = 0;

        loop {
            let drunkard_idx = map.point2d_to_index(drunkard_position);
            map.tiles[drunkard_idx] = TileType::Floor;

            match rng.range(0, 4) {
                0 => drunkard_position.x -= 1,
                1 => drunkard_position.x += 1,
                2 => drunkard_position.y -= 1,
                _ => drunkard_position.y += 1,
            }

            if !map.in_bounds(drunkard_position) {
                break;
            }

            distance_staggered += 1;
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}
