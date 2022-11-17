use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq, Eq)]
// The type of Tiles the player may encounter. If we wanted to add other
// obstacles, we would start by defining them here.
pub enum TileType {
    Exit,
    Floor,
    Wall,
}

/// Calculate the tile index
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

/// The complete map, revealed and hidden
pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        }
    }

    /// Is this point inside the map?
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// Test if a point is inside the map.
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    /// Check if a user can enter a tile
    ///
    /// Right now, the player can only enter a Floor tile or an Exit.
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point)
            && (self.tiles[map_idx(point.x, point.y)] == TileType::Floor
                || self.tiles[map_idx(point.x, point.y)] == TileType::Exit)
    }

    /// Determines if an exit from a tile is valid
    ///
    /// This checks to see if a point is a valid next move for an entity, i.e.,
    /// is this a floor or exit tile?
    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for Map {
    /// For a given tile index, returns a vector containing possible next moves.
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();

        // Get the Point mapping to this map index
        let location = self.index_to_point2d(idx);

        // 1.0 is the cost of navigating the route. A lower cost makes them more likely to be chosen.
        // A swamp tile, for example, would have a higher navigating cost while
        // a road would have a lower cost.

        // West
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        // East
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        // North
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        // South
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }
        exits
    }

    /// Get the remaining distance between two tiles, i.e., between player and exit
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    /// Is the Tile opaque or not?
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }
}
