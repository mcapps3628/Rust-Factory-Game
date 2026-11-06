use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub money: i32,
    pub xp: i32,
    pub research_xp: i32,
}