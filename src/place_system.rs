use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::grid_system::{place_tile, world_to_grid, Grid};
use crate::score_system::Score;
use crate::tile_system::TileType;

#[derive(Resource, Default)]
pub struct BuildMode(pub bool);

pub fn mouse_place_tile(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut grid: ResMut<Grid>,
    mut score: ResMut<Score>,
    mut build_mode: ResMut<BuildMode>,
) {
    if key.just_pressed(KeyCode::ShiftRight) {
        build_mode.0 = !build_mode.0;
    }

    // 4. Guard clauses: check both build mode and mouse input
    if !build_mode.0 || !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    // Cursor position in window (pixel) coordinates; None if the cursor is outside the window
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Window coords to world coords through the camera
    let (camera, camera_transform) = *camera;
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    // World coords to grid cell (None if outside the grid)
    let Some(grid_pos) = world_to_grid(world_pos) else {
        return;
    };

    place_tile(&mut commands, &mut grid, &mut score, TileType::Tile1, grid_pos);
}