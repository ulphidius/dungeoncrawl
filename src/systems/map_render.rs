use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
) {
    let mut field_of_view = <&FieldOfView>::query().filter(component::<Player>());
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(MAP_LAYER);

    let player_field_of_view = field_of_view.iter(ecs).nth(0).unwrap();

    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x ..= camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);

            let idx = map_idx(x, y);
            if map.in_bounds(pt) 
                && player_field_of_view.visible_tiles.contains(&pt)
                || map.revealed_tiles[idx] {
                let tint = if player_field_of_view.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARK_GRAY
                };

                let (color, glyph) = match map.tiles[idx] {
                    TileType::Floor => (ColorPair { fg: tint, bg: BLACK }, to_cp437('.')),
                    TileType::Wall => (ColorPair { fg: tint, bg: BLACK }, to_cp437('#')),
                };

                draw_batch.set(
                    pt - offset,
                    color,
                    glyph,
                );
            }
        }
        draw_batch.submit(0).expect("batch submit fail");
    }
}
