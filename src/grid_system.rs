use bevy::prelude::*;
use crate::score_system::Score;
use crate::tile_system::TileType;
use crate::processor_tile::{spawn_processor, Processor};

pub const GRID_WIDTH: u32 = 32;
pub const GRID_HEIGHT: u32 = 32;
pub const TILE_SIZE: u32 = 64;

#[derive(Resource)]
pub struct Grid {
    pub matrix: [[Option<GridSlot>; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
}

impl Default for Grid {
    fn default() -> Self {
        Self { matrix: [[None; GRID_WIDTH as usize]; GRID_HEIGHT as usize] }
    }
}

#[derive(Clone, Copy)]
pub struct GridSlot {
    pub tile_type: TileType,
    pub entity: Entity,
}

// Returns true if the tile was placed.
pub fn place_tile(
    commands: &mut Commands,
    grid: &mut Grid,
    score: &mut Score,
    tile_type: TileType,
    grid_pos: IVec2,
) -> bool {
    if !in_bounds(grid_pos) {
        return false;
    }

    let (x, y) = (grid_pos.x as usize, grid_pos.y as usize);
    if grid.matrix[y][x].is_some() {
        return false;
    }

    // Build the tile's data
    let processor = match tile_type {
        TileType::Tile1 => Processor::tile1(),
    };

    // Buy check
    let cost = processor.cost;
    if score.money < cost {
        return false;
    }

    let world_pos = grid_to_world(grid_pos);
    let entity = spawn_processor(commands, processor, world_pos);

    score.money -= cost;
    grid.matrix[y][x] = Some(GridSlot { tile_type, entity });
    true
}

// Convert grid to world
pub fn grid_to_world(grid_location: IVec2) -> Vec2 {
    let world_x = (grid_location.x as f32 - (GRID_WIDTH as f32 / 2.0)) * TILE_SIZE as f32 + (TILE_SIZE as f32 / 2.0);
    let world_y = (grid_location.y as f32 - (GRID_HEIGHT as f32 / 2.0)) * TILE_SIZE as f32 + (TILE_SIZE as f32 / 2.0);

    Vec2::new(world_x, world_y)
}

// Convert world to grid
pub fn world_to_grid(world_pos: Vec2) -> Option<IVec2> {
    let x = ((world_pos.x + (GRID_WIDTH as f32 * TILE_SIZE as f32 / 2.0)) / TILE_SIZE as f32).floor() as i32;
    let y = ((world_pos.y + (GRID_HEIGHT as f32 * TILE_SIZE as f32 / 2.0)) / TILE_SIZE as f32).floor() as i32;

    if x >= 0 && x < GRID_WIDTH as i32 && y >= 0 && y < GRID_HEIGHT as i32 {
        Some(IVec2::new(x, y))
    } else {
        None // Out of bounds
    }
}

// Bounds check
fn in_bounds(pos: IVec2) -> bool {
    pos.x >= 0 && pos.y >= 0 && pos.x < GRID_WIDTH as i32 && pos.y < GRID_HEIGHT as i32
}
