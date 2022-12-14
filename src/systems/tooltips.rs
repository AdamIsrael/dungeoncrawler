use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point, // (1)
    #[resource] camera: &Camera,   // (2)
) {
    let mut positions = <(Entity, &Point, &Name)>::query(); // (3)

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset; // (4)
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions
        .iter(ecs) // (5)
        // filter out positions that aren't rendered and aren't visible
        .filter(|(_, pos, _)| **pos == map_pos && player_fov.visible_tiles.contains(pos))
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_pos * 4; // (7)
            let display = if let Ok(health) = ecs
                .entry_ref(*entity) // (8)
                .unwrap()
                .get_component::<Health>()
            {
                format!("{} : {} hp", &name.0, health.current) // (9)
            } else {
                // (10)
                name.0.clone()
            };
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}
