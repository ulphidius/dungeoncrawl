use crate::prelude::*;
pub const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<TileType>
}

impl Map {
    pub fn new(size: usize) -> Self {
        return Self {
            tiles: vec![TileType::Floor; size],
        };
    }

    pub fn render(&self, context: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);

                match self.tiles[idx] {
                    TileType::Floor => context.set(
                        x,
                        y,
                        YELLOW,
                        BLACK,
                        to_cp437('.'),
                    ),
                    TileType::Wall => context.set(
                        x,
                        y,
                        GREEN,
                        BLACK,
                        to_cp437('#'),
                    ),
                }
            }
        }
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
}

/// striding function: raw_first encoding.
/// if it perform poorly them chan it by Morton Encoding.
pub fn map_idx(x: i32, y: i32) -> usize {
    return (y * SCREEN_WIDTH + x) as usize;
}
