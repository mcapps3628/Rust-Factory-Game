use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub money: u32,
    pub xp: u32,
    pub research_xp: u32,
}

pub fn buy(
    cost: u32, 
    mut score: Score,
) -> bool {
    if cost > score.money {
        score.money -= cost;
        true
    } else {
        false
    }
}