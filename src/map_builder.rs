pub use crate::prelude::*;

const NUMBER_OF_ROOMS: usize = 20;
const MIN_ROOM_SIZE: i32 = 2;
const MAX_ROOM_SIZE: i32 = 10;

#[derive(Debug)]
pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(map_size: usize, rng: &mut RandomNumberGenerator) -> Self {
        let mut builder = Self {
            map: Map::new(map_size),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        builder.fill(TileType::Wall);
        builder.build_random_rooms(rng);
        builder.build_corridors(rng);
        builder.player_start = builder.rooms[0].center();

        return builder;
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUMBER_OF_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - MAX_ROOM_SIZE),
                rng.range(1, SCREEN_HEIGHT - MAX_ROOM_SIZE),
                rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE),
                rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE),
            );

            let mut overlap = false;
            if self.rooms.len() == 0 {
                self.rooms.push(room);
                continue;
            }

            for r in self.rooms.clone() {
                if r.intersect(&room) {
                    overlap = true;
                }

                if !overlap {
                    room.for_each(|p| {
                        if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                            self.map.tiles[map_idx(p.x, p.y)] = TileType::Floor;
                        }
                    });
                    self.rooms.push(room);
                }
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};
        for y in min(y1, y2) ..= max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2) ..= max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            dbg!(room);
            let prev = rooms[i-1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            }else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}
