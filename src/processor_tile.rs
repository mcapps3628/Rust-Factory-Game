use bevy::prelude::*;
use crate::grid_system::TILE_SIZE;
use crate::tile_system::{Recipe, TileType};

#[derive(Component)]
pub struct Processor {
    pub tile_type: TileType,
    pub cost: u32,
    pub sell_price: u32,
    pub recipe: Recipe,
    pub speed_multiplier: f32,
}

impl Processor {
    pub fn tile1() -> Self {
        Processor {
            tile_type: TileType::Tile1,
            cost: 100,
            sell_price: 90,
            recipe: Recipe {
                inputs: vec![],
                outputs: vec![],
                craft_time: 1.0,
            },
            speed_multiplier: 1.0,
        }
    }
}

pub fn spawn_processor(
    commands: &mut Commands,
    processor: Processor,
    world_pos: Vec2, //TODO: Add texture
) -> Entity {
    commands.spawn((
        processor,
        Sprite {
            color: Color::srgb(1.0, 0.6, 1.0),
            custom_size: Some(Vec2::splat(TILE_SIZE as f32)),
            ..default()
        },
        Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
    )).id()
}