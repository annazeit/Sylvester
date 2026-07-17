use bevy::prelude::*;
use bevy::color::Mix;

use crate::model::game_model::AppState;
use crate::snake_model::{SnakeModel, SnakeSpineNodeType};
use crate::creature_body_evolution::{SCALE_TRANSITION_DURATION, ease_smoothstep};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BackgroundTransition>();
        app.add_systems(Update, background_color_update.run_if(in_state(AppState::Playing)));
    }
}

// Background color at each evolution tier - darker/deeper as the creature grows,
// evoking descending further in, the same way flOw's water darkens in deeper layers.
pub fn background_color_for_tier(tier: SnakeSpineNodeType) -> Color {
    match tier {
        SnakeSpineNodeType::Small => Color::srgb_u8(43, 44, 47), // today's existing default background
        SnakeSpineNodeType::Medium => Color::srgb(0.05, 0.12, 0.18),
        SnakeSpineNodeType::Big => Color::srgb(0.02, 0.04, 0.09),
    }
}

// Tracks the in-progress background fade independently of SnakeModel - this is a
// rendering/environment concern, not part of the creature's own body state.
#[derive(Resource)]
struct BackgroundTransition {
    tier: SnakeSpineNodeType,
    start_color: Color,
    elapsed: f32,
}

impl Default for BackgroundTransition {
    fn default() -> Self {
        Self {
            tier: SnakeSpineNodeType::Small,
            start_color: background_color_for_tier(SnakeSpineNodeType::Small),
            elapsed: SCALE_TRANSITION_DURATION, // starts settled
        }
    }
}

fn background_color_update(
    snake_query: Query<&SnakeModel>,
    mut transition: ResMut<BackgroundTransition>,
    mut clear_color: ResMut<ClearColor>,
    time: Res<Time>,
) {
    let Some(snake) = snake_query.iter().next() else { return; };

    if snake.evolution_tier != transition.tier {
        transition.start_color = clear_color.0;
        transition.tier = snake.evolution_tier;
        transition.elapsed = 0.0;
    }

    let target_color = background_color_for_tier(transition.tier);
    if transition.elapsed < SCALE_TRANSITION_DURATION {
        let t = ease_smoothstep(transition.elapsed / SCALE_TRANSITION_DURATION);
        clear_color.0 = transition.start_color.mix(&target_color, t);
        transition.elapsed += time.delta_seconds();
    } else {
        clear_color.0 = target_color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_tier_matches_todays_default_background() {
        assert_eq!(background_color_for_tier(SnakeSpineNodeType::Small), Color::srgb_u8(43, 44, 47));
    }

    #[test]
    fn medium_tier_is_a_darker_blue_teal() {
        assert_eq!(background_color_for_tier(SnakeSpineNodeType::Medium), Color::srgb(0.05, 0.12, 0.18));
    }

    #[test]
    fn big_tier_is_a_near_black_deep_blue() {
        assert_eq!(background_color_for_tier(SnakeSpineNodeType::Big), Color::srgb(0.02, 0.04, 0.09));
    }
}
