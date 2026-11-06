use bevy::prelude::*;

mod grid_system;
mod tile_system;
mod score_system;
mod item_system;
mod processor_tile;
mod place_system;

use score_system::Score;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<grid_system::Grid>()
        .init_resource::<score_system::Score>()
        .init_resource::<place_system::BuildMode>()
        .add_systems(Startup, setup)
        .add_systems(Update, place_system::mouse_place_tile)
        .run();
}

fn setup(mut commands: Commands, mut score: ResMut<Score>) {
    commands.spawn(Camera2d);

    score.money = 500; // Example starter money
}