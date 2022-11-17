use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(ecs: &mut SubWorld, #[resource] map: &Map) {
    // read the Point (player) that can write to the Field of View
    let mut views = <(&Point, &mut FieldOfView)>::query();

    // mark new field of view tiles as visible
    views
        .iter_mut(ecs)
        // filter out tiles we already know are visible
        .filter(|(_, fov)| fov.is_dirty)
        // mark the tiles as visible
        .for_each(|(pos, mut fov)| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);
            fov.is_dirty = false;
        });
}
