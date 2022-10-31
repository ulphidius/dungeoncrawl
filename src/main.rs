mod components;
mod spawner;
mod systems;
mod map;
mod map_builder;
mod camera;

mod prelude {
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
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
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();

        let map_builder = MapBuilder::new(NUM_TILES, &mut rng);
        spawn_player(&mut ecs, map_builder.player_start);

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        return Self{
            ecs,
            resources,
            systems: build_scheduler(),
        };
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();

        ctx.set_active_console(1);
        ctx.cls();

        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("fail to render batch");
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
