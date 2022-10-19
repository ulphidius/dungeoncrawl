mod map;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const GAME_TITLE: &str = "Dungeon Crawler";
    pub const GAME_FPS: f32 = 30.0;
    pub use crate::map::*; 
}

use prelude::*;

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        return Self {
            map: Map::new(NUM_TILES),
        };
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title(GAME_TITLE)
        .with_fps_cap(GAME_FPS)
        .build()?;

    return main_loop(context, State::new());
}
