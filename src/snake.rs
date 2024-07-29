use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::palettes::css::{GREEN, WHITE, YELLOW};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Gizmos, KeyCode, Query, Res};
use bevy::prelude::Time;
use std::f32::*;
use rand::Rng;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, snake_start);
        app.add_systems(Update, snake_update);
        app.add_systems(Startup, food_start);
        app.add_systems(Update, food_draw);
    }
}
fn snake_start (mut commands: Commands) {
    for i in 0..1 {
        commands.spawn(SnakeHead {
            head_pos: Vec2::new(0.0, i as f32 * -100.0),
            head_direction_angle: 0.0,
            head_radius: 50.0,
            // distance_from_last_turn: 0.0,
            // direction_changes: vec![],
            movement_speed: 150.0,
            rotation_speed_in_degrees: 3.0,
        });
    }}

fn food_start (mut commands: Commands) {
    for _ in 0..3 {
        let pos: Vec2 = {
            let x = rand::thread_rng().gen_range(1..=100) as f32;
            let y = rand::thread_rng().gen_range(1..=100) as f32;
            Vec2::new(x, y)
        };
        commands.spawn(Food {
            food_pos: pos,
            radius: 10.0
        });
    }
}

fn food_is_eaten_by_any_snake(food: &Food, snake_query: &mut Query<&mut SnakeHead>) -> bool {
    for snake in snake_query {
        if snake_eats_food(&snake, food) {
            return true;
        }
    }
    return false;
}

fn food_draw(
    mut gizmos: Gizmos,
    mut food_query: Query<&mut Food>,
    mut snake_query: Query<&mut SnakeHead>,
) {
    for food in &mut food_query {
        let color = {
            if food_is_eaten_by_any_snake(&food, &mut snake_query) { GREEN }
            else { WHITE }
        };

        gizmos.circle_2d(food.food_pos, food.radius, color);
    }
}

#[derive(Component)]
struct SnakeHead {
    head_pos: Vec2,
    head_direction_angle: f32,
    head_radius: f32,
    // distance_from_last_turn: f32,
    // direction_changes: Vec<DirectionChange>,
    ///linear speed in meters per second
    movement_speed: f32,
    ///rotation speed in degrees per second. this value defines how quickly the object changes direction
    rotation_speed_in_degrees: f32
}

#[derive(Component)]
pub struct Food {
    food_pos: Vec2,
    radius: f32
}

fn draw_node(gizmos: &mut Gizmos, position: Vec2, radius: f32) {
    gizmos.circle_2d(position, radius, YELLOW);
    gizmos.circle_2d(position, radius / 2.0, YELLOW);
}

fn keyboard_movement(keyboard_input: &Res<ButtonInput<KeyCode>>, snake: &SnakeHead) -> f32 {
    let unit: f32 = {
        if keyboard_input.pressed(KeyCode::ArrowUp) { 1.0 }
        else if keyboard_input.pressed(KeyCode::ArrowDown) { -1.0 }
        else { 0.0 }
    };
    unit * snake.movement_speed
}

fn keyboard_rotation(keyboard_input: &Res<ButtonInput<KeyCode>>, snake: &SnakeHead, time: &Res<Time>) -> f32 {
    let unit: f32 = {
        if keyboard_input.pressed(KeyCode::ArrowRight) { 1.0 }
        else if keyboard_input.pressed(KeyCode::ArrowLeft) { -1.0 }
        else { 0.0 }
    };
    consts::PI / 180.0 * snake.rotation_speed_in_degrees * unit * time.delta_seconds()
}

fn draw_tail(gizmos: &mut Gizmos, radius: f32, snake: &SnakeHead){
    let mut distance = radius * 2.0;
    for i in 1..3 {
        let shift_from_head: Vec2 = {
            let x_tail = f32::sin(snake.head_direction_angle - consts::PI) * distance;
            let y_tail = f32::cos(snake.head_direction_angle - consts::PI) * distance;
            Vec2::new(x_tail, y_tail)
        };

        let tail_pos = snake.head_pos + shift_from_head;
        let tail_radius = radius - (20.0 * i as f32);
        distance += 75.0;
        draw_node(gizmos, tail_pos, tail_radius);
    }
}
fn snake_eats_food(
    snake: &SnakeHead,
    food: &Food
) -> bool {
    let distance_vector = snake.head_pos - food.food_pos;
    let distance_between = ((distance_vector.x * distance_vector.x) + (distance_vector.y * distance_vector.y)).sqrt();
    distance_between < food.radius + snake.head_radius
}
fn snake_update (
    mut gizmos: Gizmos,
    mut snake_query: Query<&mut SnakeHead>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut snake in &mut snake_query {
        snake.head_direction_angle += keyboard_rotation(&keyboard_input, &snake, &time) * (snake.movement_speed / 4.0);

        let head_move = {
            let movement = keyboard_movement(&keyboard_input, &snake);
            let x_head = f32::sin(snake.head_direction_angle) * movement * time.delta_seconds();
            let y_head = f32::cos(snake.head_direction_angle) * movement * time.delta_seconds();
            Vec2::new(x_head, y_head)
        };

        snake.head_pos += head_move;

        draw_node(&mut gizmos, snake.head_pos, snake.head_radius);

        draw_tail(&mut gizmos, snake.head_radius, &snake);
    }
}