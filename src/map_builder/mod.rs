use crate::prelude::*;
mod empty;
mod rooms;
use empty::EmptyArchitect;
use rooms::RoomsArchitect;

pub const NUMBER_OF_ROOMS: usize = 20;
const MAX_DIJKSTRA_DEPTH: f32 = 1024.0;
const UNREACHABLE: &f32 = &f32::MAX;

pub enum ArchitectAlgorithm {
    Empty,
    Rooms,
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
}

