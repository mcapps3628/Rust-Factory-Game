// TILE INFO

use bevy::prelude::*;
use crate::item_system::ItemStack;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Tile1,
}

#[derive(Debug, Clone)]
pub struct Recipe {
    pub inputs: Vec<ItemStack>,
    pub outputs: Vec<ItemStack>,
    pub craft_time: f32, // seconds at speed_multiplier = 1.0
}



