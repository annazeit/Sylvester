use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::*;
use rand::Rng;
use std::f32::*;
use std::f64::consts::PI;

use crate::snake_model::SnakeModel;
use crate::grid::*;
use super::STARTING_SNAKE_SIZE;
use super::bound::{Bound, BASE_BOUND_RADIUS};
use super::score::{Score, add_point};

#[derive(Component)]
pub(super) struct Food {
    pos: Vec2,
    direction: f32,
    radius: f32,
    is_poisonous: bool,
}

// Chance any given food spawn/respawn is poisonous.
const POISON_CHANCE: f64 = 0.2;
// Size lost when eating poisonous food - a real setback, not devastating.
const POISON_SIZE_PENALTY: f32 = 3.0;
// Floor so the creature never shrinks to nothing.
const MIN_SNAKE_SIZE: f32 = 1.0;
// Warning tint applied to poisonous food's sprite so it's learnable/avoidable.
// Bright and saturated (not dark) so it reads clearly against the dark background instead of blending in.
const POISON_SPRITE_COLOR: Color = Color::srgb(0.2, 1.0, 0.3);

const BASE_FOOD_COUNT: usize = 5; // today's fixed count
const SIZE_PER_EXTRA_FOOD: f32 = 5.0;
const MAX_FOOD_COUNT: usize = 20; // cap so entity count doesn't grow unbounded over a long session
// Fraction of the boundary radius food spawns within, keeping it off the very edge.
const FOOD_SPAWN_MARGIN_FACTOR: f32 = 0.6;

pub(super) fn target_food_count(snake_size: f32) -> usize {
    let extra = ((snake_size - STARTING_SNAKE_SIZE).max(0.0) / SIZE_PER_EXTRA_FOOD) as usize;
    (BASE_FOOD_COUNT + extra).min(MAX_FOOD_COUNT)
}

pub(super) fn food_start(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..BASE_FOOD_COUNT {
        spawn_food(&mut commands, &asset_server, BASE_BOUND_RADIUS);
    }
}

fn spawn_food(commands: &mut Commands, asset_server: &Res<AssetServer>, bound_radius: f32) {
    let food_image_size = 100.0;
    let radius = 10.0;
    let scale = (radius * 2.0) / food_image_size;
    let pos = new_food_position(bound_radius);
    let is_poisonous = new_food_is_poisonous();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Food.png"),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::new(scale, scale, scale)),
            sprite: Sprite { color: food_sprite_color(is_poisonous), ..default() },
            ..default()
        },
        Food {
            pos,
            direction: new_food_direction(rand::thread_rng().gen_range(0.0..= consts::PI * 2.0) as f32),
            radius,
            is_poisonous,
        }
    ));
}

// Tops up the food pool toward target_food_count as the snake grows, mirroring
// ensure_body_capacity's pattern in creature_body_evolution.rs.
pub(super) fn ensure_food_capacity(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    snake_query: Query<&SnakeModel>,
    existing_food_query: Query<&Food>,
    bound_query: Query<&Bound>,
) {
    let Some(snake) = snake_query.iter().next() else { return; };
    let target = target_food_count(snake.size);
    let current = existing_food_query.iter().count();
    if current < target {
        let bound_radius = bound_query.iter().next().map(|b| b.radius).unwrap_or(BASE_BOUND_RADIUS);
        for _ in current..target {
            spawn_food(&mut commands, &asset_server, bound_radius);
        }
    }
}

fn new_food_position(bound_radius: f32) -> Vec2 {
    let spawn_radius = bound_radius * FOOD_SPAWN_MARGIN_FACTOR;
    let x = rand::thread_rng().gen_range(-spawn_radius..=spawn_radius);
    let y = rand::thread_rng().gen_range(-spawn_radius..=spawn_radius);
    Vec2::new(x, y)
}
// Picks a new wander direction roughly opposite the last one (+/- a small random
// wobble), used both when food is eaten/respawned and when it bounces off the bound.
fn new_food_direction(last_direction: f32) -> f32 {
    let num = rand::thread_rng().gen_range(-10.0..= 10.0) as f32;
    let new_direction = last_direction - consts::PI + (num / 10.0);
    new_direction
}
fn new_food_is_poisonous() -> bool {
    rand::thread_rng().gen_bool(POISON_CHANCE)
}
// Maps poison state to the food's actual on-screen sprite tint (the warning color
// for poisonous food, or white/untouched for normal food's natural Food.png look).
fn food_sprite_color(is_poisonous: bool) -> Color {
    if is_poisonous { POISON_SPRITE_COLOR } else { Color::WHITE }
}

// True when the snake's head circle overlaps the food's circle.
fn snake_eats_food(
    snake: &SnakeModel,
    food: &Food
) -> bool {
    let distance_vector = snake.head_pos - food.pos;
    let distance_between = ((distance_vector.x * distance_vector.x) + (distance_vector.y * distance_vector.y)).sqrt();
    distance_between < food.radius + snake.head_radius
}
// Redirects food that has wandered too close to the boundary edge, keeping it inside the play area.
fn food_on_bound(food: &mut Food, bound_query: &Query<&mut Bound>) {
    for bound in bound_query {
        let origin = Vec2::new(0.0, 0.0);
        let distance_from_origin_to_food: f32 = {
            let distance_vector = origin - food.pos;
            ((distance_vector.x * distance_vector.x) + (distance_vector.y * distance_vector.y)).sqrt()
        };
        if distance_from_origin_to_food > (bound.radius - (food.radius * 2.0 )) {
            food.direction = new_food_direction(food.direction)
        }
    }
}

fn draw_food(food: &mut Food, gizmos: &mut Gizmos, query: &Query<&GridVisualDiagnostic>) {
    let food_move = {
        let x = f32::cos(food.direction);
        let y = f32::sin(food.direction);
        Vec2::new(x, y)
    };
    food.pos += food_move;

    if grid_draw_visual_diagnostics_info(&query) {
        gizmos.circle_2d(food.pos, food.radius, food_sprite_color(food.is_poisonous));
    }
}

pub(super) fn food_update(
    mut gizmos: Gizmos,
    bound_query: Query<&mut Bound>,
    mut food_query: Query<(&mut Food, &mut Transform, &mut Sprite)>,
    mut snake_query: Query<&mut SnakeModel>,
    mut score_query: Query<(&mut Text, &mut Score)>,
    query: Query<&GridVisualDiagnostic>,

) {
    let bound_radius = bound_query.iter().next().map(|b| b.radius).unwrap_or(BASE_BOUND_RADIUS);

    for (mut food, mut transform, mut sprite) in &mut food_query {
        for mut snake in &mut snake_query {
            // eating: poisonous food shrinks the snake, normal food grows it and bumps score;
            // either way the food respawns elsewhere with a freshly rolled poison state
            if snake_eats_food(&snake, &food) {
                if food.is_poisonous {
                    snake.size = (snake.size - POISON_SIZE_PENALTY).max(MIN_SNAKE_SIZE);
                } else {
                    snake.size += 1.0;
                    add_point(&mut score_query);
                }

                food.direction = new_food_direction(food.direction);
                food.pos = new_food_position(bound_radius);
                food.is_poisonous = new_food_is_poisonous();
                sprite.color = food_sprite_color(food.is_poisonous);
                break;
            }
        }

        draw_food(&mut food, &mut gizmos, &query);

        food_on_bound(&mut food, &bound_query);

        transform.translation = Vec3::new(food.pos.x, food.pos.y, 0.0);
        transform.rotation = Quat::from_rotation_z(food.direction + consts::PI / 2.0 + consts::PI);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poisonous_food_gets_warning_tint() {
        assert_eq!(food_sprite_color(true), POISON_SPRITE_COLOR);
    }

    #[test]
    fn normal_food_gets_untouched_white_tint() {
        assert_eq!(food_sprite_color(false), Color::WHITE);
    }

    #[test]
    fn food_count_matches_today_at_and_below_starting_size() {
        assert_eq!(target_food_count(STARTING_SNAKE_SIZE), BASE_FOOD_COUNT);
        assert_eq!(target_food_count(0.0), BASE_FOOD_COUNT);
    }

    #[test]
    fn food_count_grows_beyond_starting_size() {
        assert_eq!(target_food_count(STARTING_SNAKE_SIZE + SIZE_PER_EXTRA_FOOD * 2.0), BASE_FOOD_COUNT + 2);
    }

    #[test]
    fn food_count_is_capped_at_max() {
        assert_eq!(target_food_count(STARTING_SNAKE_SIZE + SIZE_PER_EXTRA_FOOD * 1000.0), MAX_FOOD_COUNT);
    }
}
