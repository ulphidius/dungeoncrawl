use crate::prelude::*;
mod empty;
mod rooms;
mod automata;
use empty::EmptyArchitect;
use rooms::RoomsArchitect;
use automata::CellularAutomataArchitect;

pub const NUMBER_OF_ROOMS: usize = 20;
pub const NUMBER_OF_MONSTER: usize = 50;
const PLAYER_MONSTER_SPAWN_DISTANCE: f32 = 10.0;
const MAX_DIJKSTRA_DEPTH: f32 = 1024.0;
const UNREACHABLE: &f32 = &f32::MAX;

pub enum ArchitectAlgorithm {
    Empty,
    Rooms,
    Automata,
}

trait MapArchitect {
    fn new(&mut self, map_size: usize, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

#[derive(Debug)]
pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl MapBuilder {
    pub fn new(map_size: usize, algorithm: ArchitectAlgorithm, rng: &mut RandomNumberGenerator) -> Self {
        return match algorithm {
            ArchitectAlgorithm::Empty => EmptyArchitect{}.new(map_size, rng),
            ArchitectAlgorithm::Rooms => RoomsArchitect{}.new(map_size, rng),
            ArchitectAlgorithm::Automata => CellularAutomataArchitect{}.new(map_size, rng),
        };
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            MAX_DIJKSTRA_DEPTH,
        );

        self.map.index_to_point2d(
            dijkstra_map.map.iter()
                .enumerate()
                .filter(|(_, distance)| *distance < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap().0
        )
    }

    fn spawn_monster(&self, start: &Point, number_of_monsters: usize, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        let mut spawnable_tiles: Vec<Point> = self.map.tiles.iter()
            .enumerate()
            .filter(|(idx, tile)| 
                **tile == TileType::Floor
                && DistanceAlg::Pythagoras.distance2d(
                    *start,
                    self.map.index_to_point2d(*idx),
                ) > PLAYER_MONSTER_SPAWN_DISTANCE
            )
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();
        let mut spawns = Vec::new();
        for _ in 0..number_of_monsters {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            spawnable_tiles.remove(target_index);
        }

        return spawns;
    }
}

