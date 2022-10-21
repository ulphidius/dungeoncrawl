use crate::prelude::*;

pub struct Player {
    pub position: Point
}

impl Player {
    pub fn new(position: Point) -> Self {
        return Self {
            position: position,
        };
    }

    pub fn render(&self, context: &mut BTerm) {
        context.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, context: &mut BTerm, map: &Map) {
        if let Some(key) = context.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };

            let new_position = self.position + delta;
            if map.can_enter_in_tile(new_position) {
                self.position = new_position;
            }
        }
    }
}
