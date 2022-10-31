use crate::prelude::*;

const CONSOLE_TO_RENDER: usize = 1;
const SORT_ORDER: usize = 5000;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(CONSOLE_TO_RENDER);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(
                *pos - offset,
                render.color,
                render.glyph,
            );
        });
    draw_batch.submit(SORT_ORDER).expect("batch submit fail");
}
