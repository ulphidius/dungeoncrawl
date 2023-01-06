use crate::prelude::*;
use super::MapArchitect;

pub struct CellularAutomataArchitect{}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, map_size: usize, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut map_builder = MapBuilder {
            map: Map::new(map_size),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };
        self.random_noise(rng, &mut map_builder.map);
        for _ in 0..10 {
            self.iteration(&mut map_builder.map);
        }
        let start = self.find_start(&map_builder.map);
        map_builder.monster_spawns = map_builder.spawn_monster(&start, NUMBER_OF_MONSTER, rng);
        map_builder.player_start = start;
        map_builder.amulet_start = map_builder.find_most_distant();

        return map_builder;
    }
}

impl CellularAutomataArchitect {
    fn random_noise(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut()
            .for_each(|tile| {
                let random = rng.range(0, 100);
                if random > 55 {
                    *tile = TileType::Floor;
                    return
                }

                *tile = TileType::Wall;
            });
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;

        for x_axis in -1..=1 {
            for y_axis in -1..=1 {
                let is_current_possition = x_axis == 0 && y_axis == 0;
                if !is_current_possition && map.tiles[map_idx(x+x_axis, y+y_axis)] == TileType::Floor {
                    neighbors += 1;
                }
            }
        }

        return neighbors;
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();

        for y in 1..(SCREEN_HEIGHT - 1) {
            for x in 1..(SCREEN_WIDTH - 1) {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = map_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                    continue;
                }

                new_tiles[idx] = TileType::Floor;
            }
        }

        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map.tiles.iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(
                center, 
                map.index_to_point2d(idx),
            )))
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(&distance2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        return map.index_to_point2d(closest_point);
    }
}
