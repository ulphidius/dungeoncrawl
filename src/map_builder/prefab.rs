use crate::prelude::*;

const DIJKSTRA_MAX_DISTANCE: f32 = 2000.0;
const DIJKSTRA_MIN_DISTANCE: f32 = 20.0;
const FORTRESS_WIDTH: i32 = 12;
const FORTRESS_HEIGHT: i32 = 11; 

const FORTRESS: (&str, i32, i32) = ("
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
", FORTRESS_WIDTH, FORTRESS_HEIGHT);

pub fn apply_prefab(map_builder: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![map_builder.map.point2d_to_index(map_builder.player_start)],
        &map_builder.map,
        MAX_DIJKSTRA_DEPTH,
    );

    let mut attempts = 0;

    while placement.is_none() && attempts < 10 {
        let dimension = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - FORTRESS.1),
            rng.range(0, SCREEN_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2,
        );

        let mut can_place = false;
        dimension.for_each(|position| {
            let idx = map_builder.map.point2d_to_index(position);
            let distance = dijkstra_map.map[idx];
            if distance < DIJKSTRA_MAX_DISTANCE
                && distance > DIJKSTRA_MIN_DISTANCE 
                && map_builder.amulet_start != position {
                    can_place = true;
            }
        });

        if can_place {
            placement = Some(Point::new(dimension.x1, dimension.y1));
            let points = dimension.point_set();
            map_builder.monster_spawns.retain(|position| !points.contains(position));
        }
        attempts += 1;
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = FORTRESS.0.chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();
        let mut i = 0;
        
        for ty in dbg!(placement.y..(placement.y + FORTRESS.2)) {
            for tx in dbg!(placement.x..(placement.x + FORTRESS.1)) {
                let idx = map_idx(tx, ty);
                if i >= string_vec.len() {
                    continue;
                }  
                let c = string_vec[i];

                match c {
                    'M' => {
                        map_builder.map.tiles[idx] = TileType::Floor;
                        map_builder.monster_spawns.push(Point::new(tx, ty));
                    },
                    '-' => map_builder.map.tiles[idx] = TileType::Floor,
                    '#' => map_builder.map.tiles[idx] = TileType::Wall,
                    _ => println!("No idea what to do with [{}]", c),
                }
                i +=1;
            }
        }
    }
}

