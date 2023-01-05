use crate::prelude::*;

const SORT_ORDER: usize = 5000;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut field_of_view = <&FieldOfView>::query().filter(component::<Player>()); 
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ENTITIES_LAYER);
    let offset = Point::new(camera.left_x, camera.top_y);

    let player_field_of_view = field_of_view.iter(ecs).nth(0).unwrap();

    renderables.iter(ecs)
        .filter(|(position, _)| player_field_of_view.visible_tiles.contains(&position))
        .for_each(|(pos, render)| {
            draw_batch.set(
                *pos - offset,
                render.color,
                render.glyph,
            );
        });
    draw_batch.submit(SORT_ORDER).expect("batch submit fail");
}
