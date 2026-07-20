use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::palettes::basic::RED;
use bevy::math::Vec2;
use bevy::prelude::*;

use crate::snake_model::SnakeModel;
use crate::grid::*;
use crate::model::game_model::AppState;
use super::STARTING_SNAKE_SIZE;

pub struct BoundPlugin;

impl Plugin for BoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, bound_start);
        app.add_systems(Update, bound_update.run_if(in_state(AppState::Playing)));
        app.add_systems(Update, draw_bound);
    }
}

#[derive(Component)]
pub struct Bound {
    pub(super) pos: Vec2,
    pub(super) radius: f32,
}

const BOUND_RADIUS_GROWTH_PER_SIZE: f32 = 15.0;
pub(super) const BASE_BOUND_RADIUS: f32 = 500.0; // today's fixed value

pub(super) fn target_bound_radius(snake_size: f32) -> f32 {
    BASE_BOUND_RADIUS + (snake_size - STARTING_SNAKE_SIZE).max(0.0) * BOUND_RADIUS_GROWTH_PER_SIZE
}

fn bound_start(mut commands: Commands) {
    commands.spawn(Bound {
        pos: Vec2::new(0.0, 0.0),
        radius: BASE_BOUND_RADIUS,
    });
}

// Grows Bound.radius with the snake's size - no easing needed, since size already
// increases in small, gradual steps (+1 per food, -3 per poison).
fn bound_update(snake_query: Query<&SnakeModel>, mut bound_query: Query<&mut Bound>) {
    let Some(snake) = snake_query.iter().next() else { return; };
    let target = target_bound_radius(snake.size);
    for mut bound in &mut bound_query {
        bound.radius = target;
    }
}

fn draw_bound(
    mut gizmos: Gizmos,
    bound_query: Query<&mut Bound>,
    query: Query<&GridVisualDiagnostic>
) {
    for bound in &bound_query {
        if grid_draw_visual_diagnostics_info(&query) {
            gizmos.circle_2d(bound.pos, bound.radius, RED);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bound_radius_matches_today_at_and_below_starting_size() {
        assert_eq!(target_bound_radius(STARTING_SNAKE_SIZE), BASE_BOUND_RADIUS);
        assert_eq!(target_bound_radius(0.0), BASE_BOUND_RADIUS);
    }

    #[test]
    fn bound_radius_grows_beyond_starting_size() {
        assert_eq!(target_bound_radius(STARTING_SNAKE_SIZE + 10.0), BASE_BOUND_RADIUS + 10.0 * BOUND_RADIUS_GROWTH_PER_SIZE);
    }
}
