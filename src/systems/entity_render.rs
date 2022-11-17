use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    // query any entity that can be rendered
    let mut renderables = <(&Point, &Render)>::query();
    // query the field of view for the player
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    // get the player's field of view
    let player_fov = fov.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();

    draw_batch.target(1);

    let offset = Point::new(camera.left_x, camera.top_y);

    // draw entities that are within the player's field of view
    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}
