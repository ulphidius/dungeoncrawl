use crate::prelude::*;

const INSTRUCTION_MESSAGE: &str = "Explore the Dungeon, Cursor keys to move.";
const SORT_ORDER: usize = 10000;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER);
    draw_batch.print_centered(1, INSTRUCTION_MESSAGE);

    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH*2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!(
            "Health: {} / {}",
            player_health.current,
            player_health.max,
        ),
        ColorPair::new(WHITE, RED),
    );

    let (player, floor_number) = <(Entity, &Player)>::query().iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.floor_number)))
        .unwrap();

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: {}", floor_number + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut y = 3;
    let mut item_query = <(&Item, &Name, &Carried)>::query();
    item_query.iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(
                Point::new(3, y),
                format!("{} : {}", y - 2, &name.0)
            );
            y += 1;
        });
    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(SORT_ORDER).expect("Batch error");
}
