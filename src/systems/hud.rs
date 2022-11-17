use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
/// Display a Heads-Up Display on the user's screen
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>()); // (1)
    let player_health = health_query.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2); // (3)
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move."); // (4)

    // It would be cool if we could draw a border around the horizontal bar,
    // to make it stand out a little more.
    // let pos: Rect = Rect::with_size(0, 0, SCREEN_WIDTH * 2, 25);
    // draw_batch.draw_hollow_box(pos, ColorPair::new(BLUE, RED));

    // Draw the health bar
    draw_batch.bar_horizontal(
        Point::zero(),              // (6)
        SCREEN_WIDTH * 2,           // (7)
        player_health.current,      // (8)
        player_health.max,          // (9)
        ColorPair::new(RED, BLACK), // (10)
    );

    // TODO: Draw a second bar with player's experience

    // print the health text
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    // Current (dungeon) level - upper right corner
    let (_player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, player)| Some((*entity, player.map_level)))
        .next()
        .unwrap()
        .unwrap();

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: {}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    // Inventory - displayed in the upper left corner
    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, _player)| Some(*entity))
        .next()
        .unwrap()
        .unwrap();

    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 3;

    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });

    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    // Draw the HUD
    draw_batch.submit(10000).expect("Batch error");
}
