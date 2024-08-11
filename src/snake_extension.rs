use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::palettes::css::*;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Gizmos, KeyCode, Query, Res};
use bevy::prelude::Time;
use std::f32::*;
use crate::snake_model::*;


pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, snake_start);
        app.add_systems(Update, snake_update);
    }
}

fn snake_start (mut commands: Commands) {
    for i in snake_head_new_list() {
        commands.spawn(i);
    }}

fn draw_node(gizmos: &mut Gizmos, position: Vec2, radius: f32) {
    gizmos.circle_2d(position, radius, YELLOW);
    gizmos.circle_2d(position, radius / 2.0, YELLOW);
}

fn keyboard_movement_up_down_impure(keyboard_input: &Res<ButtonInput<KeyCode>>) -> SnakeMoveDirection {
    if keyboard_input.pressed(KeyCode::ArrowUp) { SnakeMoveDirection::Forward }
    else if keyboard_input.pressed(KeyCode::ArrowDown) { SnakeMoveDirection::Backward }
    else { SnakeMoveDirection::Stop }
}

fn keyboard_rotation(keyboard_input: &Res<ButtonInput<KeyCode>>, snake: &SnakeModel, time: &Res<Time>) -> f32 {
    let unit: f32 = {
        if keyboard_input.pressed(KeyCode::ArrowRight) { 1.0 }
        else if keyboard_input.pressed(KeyCode::ArrowLeft) { -1.0 }
        else { 0.0 }
    };
    consts::PI / 180.0 * snake.rotation_speed_in_degrees * unit * time.delta_seconds()
}

fn draw_tail(gizmos: &mut Gizmos, radius: f32, snake: &SnakeModel){
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

fn snake_update (
    mut gizmos: Gizmos,
    mut snake_query: Query<&mut SnakeModel>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut snake in &mut snake_query {
        snake.head_direction_angle += keyboard_rotation(&keyboard_input, &snake, &time) * (snake.movement_speed / 4.0);

        let head_move: SnakeModelUpdate = {
            let keyboard_up_down_input = keyboard_movement_up_down_impure(&keyboard_input);
            head_move_pure(keyboard_up_down_input, time.delta_seconds(), &snake)
        };

        snake.head_pos += head_move.new_head_pos;
        match head_move.trace_point_to_add {
            Some(point) => snake.trace.push_front(point),
            None => (),
        }
        let mut current_pos = snake.head_pos;
        let mut total_distance = 0.0;
        for i in snake.trace.iter() {
            total_distance += current_pos.distance(*i);
            gizmos.line_2d(current_pos, *i, PINK);
            current_pos = *i;
            if total_distance > 300.0 {
                break;
            }
        }
        draw_node(&mut gizmos, snake.head_pos, snake.head_radius);

        draw_tail(&mut gizmos, snake.head_radius, &snake);
    }
}