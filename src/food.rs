use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::palettes::basic::RED;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::*;
use rand::Rng;
use std::f32::*;
use std::f64::consts::PI;

use crate::snake_model::SnakeModel;
use crate::grid::*;
use crate::start::*;
use crate::model::game_model::AppState;

pub struct FoodPlugin;

#[derive(Component)]
pub struct Food {
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

// Growth is measured from the snake's starting size (snake_model_new), so nothing
// changes until the creature actually grows beyond it.
const STARTING_SNAKE_SIZE: f32 = 5.0;
const BASE_BOUND_RADIUS: f32 = 500.0; // today's fixed value
const BOUND_RADIUS_GROWTH_PER_SIZE: f32 = 15.0;
const BASE_FOOD_COUNT: usize = 5; // today's fixed count
const SIZE_PER_EXTRA_FOOD: f32 = 5.0;
const MAX_FOOD_COUNT: usize = 20; // cap so entity count doesn't grow unbounded over a long session
// Fraction of the boundary radius food spawns within, keeping it off the very edge.
const FOOD_SPAWN_MARGIN_FACTOR: f32 = 0.6;

pub fn target_bound_radius(snake_size: f32) -> f32 {
    BASE_BOUND_RADIUS + (snake_size - STARTING_SNAKE_SIZE).max(0.0) * BOUND_RADIUS_GROWTH_PER_SIZE
}

pub fn target_food_count(snake_size: f32) -> usize {
    let extra = ((snake_size - STARTING_SNAKE_SIZE).max(0.0) / SIZE_PER_EXTRA_FOOD) as usize;
    (BASE_FOOD_COUNT + extra).min(MAX_FOOD_COUNT)
}
#[derive(Component)]
pub struct Bound {
    pos: Vec2,
    radius: f32,
}

#[derive(Component)]
struct Score {
    score_num: i32
}

impl Plugin for FoodPlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), food_start);
        app.add_systems(Startup, score_start);
        app.add_systems(Startup, bound_start);
        app.add_systems(Update, food_update.run_if(in_state(AppState::Playing)));
        app.add_systems(Update, bound_update.run_if(in_state(AppState::Playing)));
        app.add_systems(Update, ensure_food_capacity.run_if(in_state(AppState::Playing)));
        app.add_systems(Update, draw_bound);
    }
}

fn bound_start(mut commands: Commands) {
    commands.spawn(Bound{
        pos: Vec2::new(0.0, 0.0),
        radius: 500.0,
    });
}
fn score_start(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("MovistarTextRegular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    let text_justification = JustifyText::Center;

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("no score", text_style.clone())
                .with_justify(text_justification),
            transform: Transform::from_xyz(0.0, 200.0, 0.0),
            ..default()
        },
        Score { score_num: 0 }
    ));
}
fn food_start (mut commands: Commands, asset_server: Res<AssetServer>) {
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

// Grows Bound.radius with the snake's size - no easing needed, since size already
// increases in small, gradual steps (+1 per food, -3 per poison).
fn bound_update(snake_query: Query<&SnakeModel>, mut bound_query: Query<&mut Bound>) {
    let Some(snake) = snake_query.iter().next() else { return; };
    let target = target_bound_radius(snake.size);
    for mut bound in &mut bound_query {
        bound.radius = target;
    }
}

// Tops up the food pool toward target_food_count as the snake grows, mirroring
// ensure_body_capacity's pattern in creature_body_evolution.rs.
fn ensure_food_capacity(
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

fn draw_bound(
    mut gizmos: Gizmos,
    bound_query: Query<&mut Bound>,
    query: Query<&GridVisualDiagnostic>
) {
    for bound in &bound_query{
        if grid_draw_visual_diagnostics_info(&query) {
            gizmos.circle_2d(bound.pos, bound.radius, RED);
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
fn food_on_bound(food: &mut Food, bound_query: &Query<&mut Bound> ) {
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
fn food_update(
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
                    for (mut text, mut score) in &mut score_query {
                        score.score_num += 1;
                        let score_string = score.score_num.to_string();
                        text.sections[0].value = format!("Score: {score_string}");
                    }
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
    fn bound_radius_matches_today_at_and_below_starting_size() {
        assert_eq!(target_bound_radius(STARTING_SNAKE_SIZE), BASE_BOUND_RADIUS);
        assert_eq!(target_bound_radius(0.0), BASE_BOUND_RADIUS);
    }

    #[test]
    fn bound_radius_grows_beyond_starting_size() {
        assert_eq!(target_bound_radius(STARTING_SNAKE_SIZE + 10.0), BASE_BOUND_RADIUS + 10.0 * BOUND_RADIUS_GROWTH_PER_SIZE);
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