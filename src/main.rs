// This generates (more) warnings related to brackets
// that I don't think I can fix here.
// #![warn(clippy::pedantic)]

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        // spawn the player
        spawn_player(&mut ecs, map_builder.player_start);

        // get/set the tile index for the exit (or amulet, depending on level)
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        // spawn the level
        spawn_level(&mut ecs, &mut rng, 0, &map_builder.monster_spawns);

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    /// Reset the game
    fn reset_game_state(&mut self) {
        /*
        A lot of this code is duplicated from new(). Is there a better way
        to reuse this code?
         */

        self.ecs = World::default();
        self.resources = Resources::default();

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        // spawn the player
        spawn_player(&mut self.ecs, map_builder.player_start);

        // get/set the tile containing the exit (or amulet, depending on level)
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        // Spawn the level
        spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns);

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    /// Advance the level
    ///
    /// Keep the player and the entities they're carrying, and reset everything
    /// else so we're carried over to a new level.
    fn advance_level(&mut self) {
        use std::collections::HashSet;

        // find the player entity
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&self.ecs)
            .next()
            .unwrap();

        // keep track of the entities we want to keep when moving to the new level
        let mut entities_to_keep = HashSet::new();
        // keep the player
        entities_to_keep.insert(player_entity);

        // find entities that the player is carrying and keep those for the next level
        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_e, carry)| carry.0 == player_entity)
            .map(|(e, _carry)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });

        let mut cb = CommandBuffer::new(&self.ecs);

        // Remove unwanted entities from the world
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.ecs);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        let mut map_level = 0;

        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });

        // If we're on the last level, spawn the amulet so that the game can be won
        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            // spawn the exit to the next level
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        // spawn the next level
        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
        );

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    /// Display the Game Over screen
    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    /// Display the Victory screen.
    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved, and you can return to your normal life.",
        );
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear the active console(s)
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();

        self.resources.insert(ctx.key);

        ctx.set_active_console(0);

        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        // check the current turn state
        let current_state = *self.resources.get::<TurnState>().unwrap();
        match current_state {
            // we're waiting for the user to hit a key
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            // the user is taking a turn
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            // the monster gets their turn
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            // Game over; the player has died
            TurnState::GameOver => {
                self.game_over(ctx);
            }
            // The player has won!
            TurnState::Victory => {
                self.victory(ctx);
            }
            // The player has reached the exit
            TurnState::NextLevel => {
                self.advance_level();
            }
        }

        // render the screen
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        // Add the resource font(s)
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
