pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
    // the map level the player is currently on
    pub map_level: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChasingPlayer;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

// Items
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item;

// Can the entity be carried in inventory?
#[derive(Clone, PartialEq, Eq)]
pub struct Carried(pub Entity);

// What happens when an item is used?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AmuletOfYala;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProvidesDungeonMap;

// Weapons

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Damage(pub i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Weapon;
