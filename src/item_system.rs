// ITEM INFO

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemType {
    Item1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemStack {
    pub item: ItemType,
    pub amount: u32,
}

#[derive(Component)]
pub struct Item {
    pub item_type: ItemType,
    pub item_name: String,
    pub cost: u32,
    pub sell_price: u32,
    pub is_sellable: bool,
}
