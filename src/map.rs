use crate::prelude::*;
pub const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

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
}

/// striding function: raw_first encoding.
/// if it perform poorly them chan it by Morton Encoding.
pub fn map_idx(x: i32, y: i32) -> usize {
    return (y * SCREEN_WIDTH + x) as usize;
}
