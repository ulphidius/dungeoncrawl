mod components;
mod spawner;
mod systems;
mod map;
mod map_builder;
mod camera;
mod turn_state;

mod prelude {
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 70;
    pub const SCREEN_HEIGHT: i32 = 40;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const TILE_HEIGHT: i32 = 32;
    pub const TILE_WIDTH: i32 = 32;
    pub const GAME_TITLE: &str = "Dungeon Crawler";
    pub const GAME_FPS: f32 = 30.0;
    pub const CHAR_WIDTH: i32 = 8;
    pub const CHAR_HEIGHT: i32 = 8;
    pub const MAP_LAYER: usize = 0;
    pub const ENTITIES_LAYER: usize = 1;
    pub const HUD_LAYER: usize = 2;
    pub const NUMBER_OF_FLOOR: u32 = 2;
}

use std::collections::HashSet;

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();

        let mut map_builder = MapBuilder::new_random(NUM_TILES, &mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        // spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_floor(
            &mut ecs,
            &mut rng,
            0,
            &map_builder.monster_spawns,
        );

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);

        return Self{
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        };
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER);
        ctx.print_color_centered(2, RED, BLACK, "Your quest end.");
        ctx.print_color_centered(4, WHITE, BLACK, "Slain by monster, your hero's journey has come to premature end.");
        ctx.print_color_centered(5, WHITE, BLACK, "The Amulet of Yala remains unclaimed, and your home town is not saved.");
        ctx.print_color_centered(8, YELLOW, BLACK, "Don't worry, you can try with a new hero.");
        ctx.print_color_centered(9, GREEN, BLACK, "Press Space to play again.");

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won !");
        ctx.print_color_centered(4, WHITE, BLACK, "You put on the Amulet of Yala and feel its power course throught yout veins.");
        ctx.print_color_centered(5, WHITE, BLACK, "Your town is saved, and you return to your normal life");
        ctx.print_color_centered(7, WHITE, BLACK, "Press Space to play again.");
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.reset_game_state();
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new_random(NUM_TILES, &mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        // spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_floor(
            &mut self.ecs,
            &mut rng,
            0,
            &map_builder.monster_spawns,
        );

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn advance_floor(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .nth(0)
            .unwrap();

        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);
        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_e, carry)| carry.0 == player_entity)
            .map(|(e, _carry)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });
        
        let mut command = CommandBuffer::new(&mut self.ecs);
        for entity in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(entity) {
                command.remove(*entity);
            }
        }
        command.flush(&mut self.ecs);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new_random(NUM_TILES, &mut rng);

        let mut floor_number: u32 = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, position)| {
                player.floor_number += 1;
                floor_number = player.floor_number;
                position.x = map_builder.player_start.x;
                position.y = map_builder.player_start.y;
            });
        if floor_number == NUMBER_OF_FLOOR {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        spawn_floor(
            &mut self.ecs,
            &mut rng,
            floor_number.try_into().unwrap(),
            &map_builder.monster_spawns,
        );
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme); 
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(MAP_LAYER);
        ctx.cls();

        ctx.set_active_console(ENTITIES_LAYER);
        ctx.cls();

        ctx.set_active_console(HUD_LAYER);
        ctx.cls();

        self.resources.insert(ctx.key);
        ctx.set_active_console(MAP_LAYER);

        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = self.resources.get::<TurnState>().unwrap().clone();

        match current_state {
            TurnState::AwaitingInput => self.input_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self.monster_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NewFloor => self.advance_floor()
        }

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
        .with_font("terminal8x8.png", CHAR_WIDTH, CHAR_HEIGHT)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH*2, SCREEN_HEIGHT*2, "terminal8x8.png")
        .build()?;

    return main_loop(context, State::new());
}
