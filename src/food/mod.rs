use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;

mod food_item;
mod bound;
mod score;

use bound::BoundPlugin;
use score::ScorePlugin;
use crate::model::game_model::AppState;

pub struct FoodPlugin;

// Growth is measured from the snake's starting size (snake_model_new), so nothing
// changes until the creature actually grows beyond it. Shared by bound.rs (play-area
// radius) and food_item.rs (food count).
const STARTING_SNAKE_SIZE: f32 = 5.0;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BoundPlugin, ScorePlugin));
        app.add_systems(OnEnter(AppState::Playing), food_item::food_start);
        app.add_systems(Update, food_item::food_update.run_if(in_state(AppState::Playing)));
        app.add_systems(Update, food_item::ensure_food_capacity.run_if(in_state(AppState::Playing)));
    }
}
