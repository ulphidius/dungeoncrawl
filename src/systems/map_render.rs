use crate::prelude::*;

const CONSOLE_TO_RENDER: usize = 0;

#[system]
pub fn map_render(
    #[resource] map: &Map,
    #[resource] camera: &Camera,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(CONSOLE_TO_RENDER);

    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x ..= camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);

            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };

                draw_batch.set(
                    pt - offset,
                    ColorPair::new(
                        WHITE,
                        BLACK,
                    ),
                    glyph,
                );
            }
        }
        draw_batch.submit(0).expect("batch submit fail");
    }
}
