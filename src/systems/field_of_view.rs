use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn field_of_view(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
) {
    let mut views = <(&Point, &mut FieldOfView)>::query();
    views.iter_mut(ecs)
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(position, mut field_of_view)| {
            field_of_view.visible_tiles = field_of_view_set(*position, field_of_view.radius, map)
        });
}
