mod map;
mod map_builder;
mod player;
mod camera;

mod prelude {
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use crate::camera::*;
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const TILE_HEIGHT: i32 = 32;
    pub const TILE_WIDTH: i32 = 32;
    pub const GAME_TITLE: &str = "Dungeon Crawler";
    pub const GAME_FPS: f32 = 30.0;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(NUM_TILES, &mut rng);
        return Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
        };
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();

        ctx.set_active_console(1);
        ctx.cls();

        self.player.update(ctx, &self.map, &mut self.camera);

        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title(GAME_TITLE)
        .with_fps_cap(GAME_FPS)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", TILE_WIDTH, TILE_HEIGHT)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    return main_loop(context, State::new());
}
