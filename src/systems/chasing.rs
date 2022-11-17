use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    // find entities that can chase the player
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    // find entities with health
    let mut positions = <(Entity, &Point, &Health)>::query();
    // find the player
    let mut player = <(&Point, &Player)>::query();

    // get the player's position and map index
    let player_pos = player.iter(ecs).next().unwrap().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    // build a dijkstra map to use in our pathfinding

    // create a vector with our player as the starting index
    let search_targets = vec![player_idx];

    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH, // (2)
        SCREEN_HEIGHT,
        &search_targets,
        map,    // (3)
        1024.0, // (4)
    );

    // find the shortest path between the player and movers, iterating over
    // entities with the `Chasing` tag.
    movers.iter(ecs).for_each(|(entity, pos, _, fov)| {
        // don't chase the player if they aren't visible to the monster
        if !fov.visible_tiles.contains(player_pos) {
            return;
        }

        // Get the entity's map index
        let idx = map_idx(pos.x, pos.y);

        // find the index of the lowest index between the entity and our player
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            // get the distance to the player
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);

            // if the player is more than 1.2 tiles away, set the destination
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };

            // determine if the entity can attack the player
            let mut attacked = false;
            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));
                    }
                    attacked = true;
                });

            // If the entity did not attack, move to its next tile
            if !attacked {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    });
}
