use crate::prelude::*;
pub const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>
}

impl Map {
    pub fn new(size: usize) -> Self {
        return Self {
            tiles: vec![TileType::Floor; size],
            revealed_tiles: vec![false, size],
        };
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        return point.x >= 0 
            && point.x < SCREEN_WIDTH 
            && point.y >= 0 
            && point.y < SCREEN_HEIGHT;
    }

    pub fn can_enter_in_tile(&self, point: Point) -> bool {
        return self.in_bounds(point)
            && self.tiles[map_idx(point.x, point.y)] == TileType::Floor;
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        return match self.in_bounds(point) {
            true => Some(map_idx(point.x, point.y)),
            false=> None,
        };
    }

    fn valid_exit(&self, location: Point, delta: Point) -> Option<usize> {
        let destination = location + delta;

        if self.in_bounds(destination) {
            if self.can_enter_in_tile(destination) {
                let idx = self.point2d_to_index(destination);
                return Some(idx);
            }

            return None;
        }

        return None;
    }

}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        return Point::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.in_bounds(pos)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exist = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exist.push((idx, 1.0));
        }

        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exist.push((idx, 1.0));
        }

        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exist.push((idx, 1.0));
        }

        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exist.push((idx, 1.0));
        }

        return exist;
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        return DistanceAlg::Pythagoras.distance2d(
            self.index_to_point2d(idx1),
            self.index_to_point2d(idx2),
        );
    }

    fn is_opaque(&self, idx: usize) -> bool {
        return self.tiles[idx] != TileType::Floor;
    }
}

/// striding function: raw_first encoding.
/// if it perform poorly them chan it by Morton Encoding.
pub fn map_idx(x: i32, y: i32) -> usize {
    return (y * SCREEN_WIDTH + x) as usize;
}
