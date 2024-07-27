use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::palettes::basic::YELLOW;
use bevy::color::palettes::css::{BLUE, GREEN, WHITE};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Gizmos, KeyCode, Mut, Query, Res};
use std::f32::*;
use rand::Rng;

pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, snake_start);
        app.add_systems(Update, snake_update);
        app.add_systems(Update, food_draw);
    }
}
fn snake_start (mut commands: Commands) {
    for i in 0..1 {
        commands.spawn(SnakeHead {
            head_pos: Vec2::new(0.0, i as f32 * -100.0),
            head_direction_angle: 0.0,
            distance_from_last_turn: 0.0,
            direction_changes: vec![],
            movement_speed: 5.0,
            rotation_speed_in_degrees: 3.0,
        });
    }}
pub fn food_draw(
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut food_query: Query<&mut Food>,
) {
    for i in 0..1 {
        commands.spawn(Food {
            food_pos: Vec2::new(1000.0, 100.0),
            movement_speed: 0.0,
        });
    }
    for mut food in &mut food_query {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen();
        let rand_num = x - 0.5;
        gizmos.circle_2d(food.food_pos, 10.0, WHITE);
    }
}

#[derive(Component)]
struct SnakeHead {
    head_pos: Vec2,
    head_direction_angle: f32,
    distance_from_last_turn: f32,
    direction_changes: Vec<DirectionChange>,
    //linear speed in meters per second
    movement_speed: f32,
    //rotation speed in degrees per second. this value defines how quickly the object changes direction
    rotation_speed_in_degrees: f32
}

#[derive(Component)]
struct DirectionChange {
    old_direction_angle: f32,
    distance_from_last_turn: f32
}

#[derive(Component)]
struct Food {
    food_pos: Vec2,
    movement_speed: f32,
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

fn keyboard_rotation(keyboard_input: &Res<ButtonInput<KeyCode>>, snake: &SnakeHead) -> f32 {
    let unit: f32 = {
        if keyboard_input.pressed(KeyCode::ArrowRight) { 1.0 }
        else if keyboard_input.pressed(KeyCode::ArrowLeft) { -1.0 }
        else { 0.0 }
    };
    consts::PI / 180.0 * snake.rotation_speed_in_degrees * unit
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
        let tail_radius = radius - (20.0 * i as f32) ;
        distance += 75.0;
        draw_node(gizmos, tail_pos, tail_radius);
    }
}

fn snake_update (
    mut gizmos: Gizmos,
    mut snake_query: Query<&mut SnakeHead>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    for mut snake in &mut snake_query {
        let head_radius = 50.0;

        snake.head_direction_angle += keyboard_rotation(&keyboard_input, &snake);;

        let head_move = {
            let movement = keyboard_movement(&keyboard_input, &snake);
            let x_head = f32::sin(snake.head_direction_angle) * movement;
            let y_head = f32::cos(snake.head_direction_angle) * movement;
            Vec2::new(x_head, y_head)
        };

        snake.head_pos += head_move;

        draw_node(&mut gizmos, snake.head_pos, head_radius);

        draw_tail(&mut gizmos, head_radius, &snake);
    }
}