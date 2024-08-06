use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Component, KeyCode, Res};
use std::f32::*;

#[derive(Component)]
pub struct SnakeHead {
    pub head_pos: Vec2,
    pub head_direction_angle: f32,
    pub head_radius: f32,
    // distance_from_last_turn: f32,
    // direction_changes: Vec<DirectionChange>,
    ///linear speed in meters per second
    pub movement_speed: f32,
    ///rotation speed in degrees per second. this value defines how quickly the object changes direction
    pub rotation_speed_in_degrees: f32
}

pub enum  SnakeMoveDirection {
    Forward,
    Backward,
    Stop
}

fn snake_head_new(i: i32) -> SnakeHead {
    SnakeHead {
        head_pos: Vec2::new(0.0, i as f32 * -100.0),
        head_direction_angle: 0.0,
        head_radius: 50.0,
        movement_speed: 150.0,
        rotation_speed_in_degrees: 3.0,
    }
}

pub fn snake_head_new_list() -> Vec<SnakeHead> {
    let mut result: Vec<SnakeHead> = Vec::new();
    for i in 0..1 {
        result.push(snake_head_new(i));
    }
    result
}

pub fn head_move_pure(keyboard_up_down_input: SnakeMoveDirection, time_delta_seconds: f32, snake: &SnakeHead) -> Vec2 {
    let keyboard_up_down_input_ratio: f32 = match keyboard_up_down_input {
        SnakeMoveDirection::Forward => { 1.0 }
        SnakeMoveDirection::Backward => { -1.0 }
        SnakeMoveDirection::Stop => { 0.0 }
    };
    let movement = keyboard_up_down_input_ratio * snake.movement_speed;
    let x_head = f32::sin(snake.head_direction_angle) * movement * time_delta_seconds;
    let y_head = f32::cos(snake.head_direction_angle) * movement * time_delta_seconds;
    Vec2::new(x_head, y_head)
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn assert_vec2_eq(a: Vec2, b: Vec2){
        let delta_max: f32 = 0.001;
        let dx = f32::abs(a.x - b.x);
        assert!(dx < delta_max);
        let dy = f32::abs(a.y - b.y);
        assert!(dy < delta_max);
    }

    #[test]
    fn no_move_because_not_key_input() {
        let actual_move = head_move_pure(SnakeMoveDirection::Stop, 10.0, &snake_head_new(0));
        let expected_move = Vec2::new(0.0, 0.0);
        assert_vec2_eq(actual_move, expected_move);
    }

    #[test]
    fn move_forward_north() {
        let mut snake = snake_head_new(0);
        snake.head_direction_angle = 0.0;
        snake.movement_speed = 3.0;
        let actual_move = head_move_pure(crate::snake_model::SnakeMoveDirection::Forward, 10.0, &snake);
        let expected_move = Vec2::new(0.0, 30.0);
        assert_vec2_eq(actual_move, expected_move);
    }

    #[test]
    fn move_backward_south() {
        let mut snake = snake_head_new(0);
        snake.movement_speed = 3.0;
        let actual_move = head_move_pure(crate::snake_model::SnakeMoveDirection::Backward, 10.0, &snake);
        let expected_move = Vec2::new(0.0, -30.0);
        assert_vec2_eq(actual_move, expected_move);
    }

    #[test]
    fn move_forward_south() {
        let mut snake = snake_head_new(0);
        snake.head_direction_angle = consts::PI;
        snake.movement_speed = 3.0;
        let actual_move = head_move_pure(crate::snake_model::SnakeMoveDirection::Forward, 10.0, &snake);
        let expected_move = Vec2::new(0.0, -30.0);
        assert_vec2_eq(actual_move, expected_move);
    }

    #[test]
    fn move_backward_north() {
        let mut snake = snake_head_new(0);
        snake.head_direction_angle = consts::PI;
        snake.movement_speed = 3.0;
        let actual_move = head_move_pure(crate::snake_model::SnakeMoveDirection::Backward, 10.0, &snake);
        let expected_move = Vec2::new(0.0, 30.0);
        assert_vec2_eq(actual_move, expected_move);
    }
}