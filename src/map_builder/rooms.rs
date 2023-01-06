use crate::prelude::*;
use super::{MapArchitect, NUMBER_OF_ROOMS};

const MIN_ROOM_SIZE: i32 = 2;
const MAX_ROOM_SIZE: i32 = 10;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self, map_size: usize, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut map_builder = MapBuilder {
            map: Map::new(map_size),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        map_builder.fill(TileType::Wall);
        self.build_random_rooms(&mut map_builder, rng);
        self.build_corridors(&mut map_builder, rng);

        map_builder.player_start = map_builder.rooms[0].center();
        map_builder.amulet_start = map_builder.find_most_distant();

        for room in map_builder.rooms.iter().skip(1) {
            map_builder.monster_spawns.push(room.center());
        }

        return map_builder;
    }
}

impl RoomsArchitect {
    fn build_random_rooms(&self, map_builder: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
        while map_builder.rooms.len() < NUMBER_OF_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - MAX_ROOM_SIZE),
                rng.range(1, SCREEN_HEIGHT - MAX_ROOM_SIZE),
                rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE),
                rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE),
            );

            let mut overlap = false;
            for r in map_builder.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        map_builder.map.tiles[idx] = TileType::Floor;
                    }
                });

                map_builder.rooms.push(room)
            }
        }
    }

    fn apply_vertical_tunnel(&self, map_builder: &mut MapBuilder, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};
        for y in min(y1, y2) ..= max(y1, y2) {
            if let Some(idx) = map_builder.map.try_idx(Point::new(x, y)) {
                map_builder.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&self, map_builder: &mut MapBuilder, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2) ..= max(x1, x2) {
            if let Some(idx) = map_builder.map.try_idx(Point::new(x, y)) {
                map_builder.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&self, map_builder: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
        let mut rooms = map_builder.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i-1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(map_builder, prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(map_builder, prev.y, new.y, new.x);
            }else {
                self.apply_vertical_tunnel(map_builder, prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(map_builder, prev.x, new.x, new.y);
            }
        }
    }
}
