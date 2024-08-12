use bevy::math::Vec2;
use bevy::prelude::{Component};
use std::collections::LinkedList;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub struct TraceItem {
    pub pos: Vec2,
    pub index: i64,
}

#[derive(Component)]
pub struct SnakeModel {
    pub head_pos: Vec2,
    pub head_direction_angle: f32,
    pub head_radius: f32,
    //linear speed in meters per second
    pub movement_speed: f32,
    //rotation speed in degrees per second. this value defines how quickly the object changes direction
    pub rotation_speed_in_degrees: f32,
    pub trace_counter: i64,
    pub trace: LinkedList<TraceItem>,
    pub tracing_step: f32,
    pub size: i32,
}

pub struct SnakeModelUpdate {
    pub new_head_pos: Vec2,
    pub trace_point_to_add: Option<Vec2>
}
pub enum  SnakeMoveDirection {
    Forward,
    Backward,
    Stop
}

fn snake_model_new(i: i32) -> SnakeModel {
    let head_pos = Vec2::new(0.0, i as f32 * -100.0);
    let trace_item = TraceItem {
        pos: head_pos,
        index: 0,
    };
    SnakeModel {
        head_pos,
        head_direction_angle: 0.0,
        head_radius: 50.0,
        movement_speed: 150.0,
        rotation_speed_in_degrees: 3.0,
        trace_counter: 0,
        trace: LinkedList::from([trace_item]),
        tracing_step: 10.0,
        size: 5
    }
}

pub fn snake_head_new_list() -> Vec<SnakeModel> {
    let mut result: Vec<SnakeModel> = Vec::new();
    for i in 0..1 {
        result.push(snake_model_new(i));
    }
    result
}

pub fn clear_extra_traces(list: &mut LinkedList<TraceItem>, max_index: i64) {
    loop {
        match list.back() {
            None => { return; },
            Some(trace_item) => {
                if trace_item.index < max_index {
                    list.pop_back();
                }
                else { return; }
            }
        }
    }
}

pub fn head_move_pure(keyboard_up_down_input: SnakeMoveDirection, time_delta_seconds: f32, snake: &SnakeModel) -> SnakeModelUpdate {
    let keyboard_up_down_input_ratio: f32 = match keyboard_up_down_input {
        SnakeMoveDirection::Forward => { 1.0 }
        SnakeMoveDirection::Backward => { -1.0 }
        SnakeMoveDirection::Stop => { 0.0 }
    };
    let movement = keyboard_up_down_input_ratio * snake.movement_speed;
    let x_head = f32::sin(snake.head_direction_angle) * movement * time_delta_seconds;
    let y_head = f32::cos(snake.head_direction_angle) * movement * time_delta_seconds;

    let new_head_move = Vec2::new(x_head, y_head);
    let last_trace_point = snake.trace.front().unwrap();

    let distance_between = (snake.head_pos + new_head_move).distance(last_trace_point.pos);
    let trace_point_to_add = {
        if distance_between >= snake.tracing_step {
            Some(snake.head_pos + new_head_move)
        }
        else { None }
    };

    SnakeModelUpdate{
        new_head_pos: new_head_move,
        trace_point_to_add
    }

}

#[cfg(test)]
mod tests {
    use std::f32::consts;
    use bevy::math::VectorSpace;

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
        let mut snake = snake_model_new(0);
        snake.tracing_step = 50.0;
        let actual_move = head_move_pure(SnakeMoveDirection::Stop, 10.0, &snake);
        let expected_move = Vec2::new(0.0, 0.0);
        assert_vec2_eq(actual_move.new_head_pos, expected_move);
        assert_eq!(actual_move.trace_point_to_add, None)
    }

    #[test]
    fn move_forward_north() {
        let mut snake = snake_model_new(0);
        snake.tracing_step = 50.0;
        snake.head_direction_angle = 0.0;
        snake.movement_speed = 3.0;
        let actual_move = head_move_pure(crate::snake_model::SnakeMoveDirection::Forward, 10.0, &snake);
        let expected_move = Vec2::new(0.0, 30.0);
        assert_vec2_eq(actual_move.new_head_pos, expected_move);
        assert_eq!(actual_move.trace_point_to_add, None)
    }

    #[test]
    fn move_backward_south() {
        let mut snake = snake_model_new(0);
        snake.tracing_step = 50.0;
        snake.movement_speed = 3.0;
        let actual_move = head_move_pure(crate::snake_model::SnakeMoveDirection::Backward, 10.0, &snake);
        let expected_move = Vec2::new(0.0, -30.0);
        assert_vec2_eq(actual_move.new_head_pos, expected_move);
        assert_eq!(actual_move.trace_point_to_add, None)
    }

    #[test]
    fn move_forward_south() {
        let mut snake = snake_model_new(0);
        snake.tracing_step = 50.0;
        snake.head_direction_angle = consts::PI;
        snake.movement_speed = 3.0;
        let actual_move = head_move_pure(crate::snake_model::SnakeMoveDirection::Forward, 10.0, &snake);
        let expected_move = Vec2::new(0.0, -30.0);
        assert_vec2_eq(actual_move.new_head_pos, expected_move);
        assert_eq!(actual_move.trace_point_to_add, None)
    }

    #[test]
    fn move_backward_north() {
        let mut snake = snake_model_new(0);
        snake.tracing_step = 50.0;
        snake.head_direction_angle = consts::PI;
        snake.movement_speed = 3.0;
        let actual_move = head_move_pure(crate::snake_model::SnakeMoveDirection::Backward, 10.0, &snake);
        let expected_move = Vec2::new(0.0, 30.0);
        assert_vec2_eq(actual_move.new_head_pos, expected_move);
        assert_eq!(actual_move.trace_point_to_add, None)
    }

    #[test]
    fn trace_track_move_up() {
        let mut snake = snake_model_new(0);
        snake.tracing_step = 50.0;
        snake.head_direction_angle = 0.0;
        snake.movement_speed = 5.0;
        let actual_move = head_move_pure(crate::snake_model::SnakeMoveDirection::Forward, 10.0, &snake);
        let expected_move = Vec2::new(0.0, 50.0);
        assert!(actual_move.trace_point_to_add.is_some());
        assert_vec2_eq(actual_move.trace_point_to_add.unwrap(), expected_move)
    }

    #[test]
    fn trace_track_move_up_with_diff_headpos() {
        let mut snake = snake_model_new(0);
        snake.tracing_step = 50.0;
        snake.head_pos = Vec2::new(0.0, 100.0);
        snake.head_direction_angle = 0.0;
        snake.movement_speed = 5.0;
        let actual_move = head_move_pure(crate::snake_model::SnakeMoveDirection::Forward, 10.0, &snake);
        let expected_move = Vec2::new(0.0, 150.0);
        assert!(actual_move.trace_point_to_add.is_some());
        assert_vec2_eq(actual_move.trace_point_to_add.unwrap(), expected_move)
    }

    #[test]
    fn trace_item_eq() {
        let a = TraceItem {
            pos: Vec2::ZERO,
            index: 3,
        };
        let b = TraceItem {
            pos: Vec2::ZERO,
            index: 3,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn trace_item_ne_different_index() {
        let a = TraceItem {
            pos: Vec2::ZERO,
            index: 3,
        };
        let b = TraceItem {
            pos: Vec2::ZERO,
            index: 5,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn clear_extra_traces_works() {
        let mut list: LinkedList<TraceItem> = LinkedList::new();
        for i in 0..10 {
            list.push_front(TraceItem {
                pos: Vec2::ZERO,
                index: i,
            })
        }
        clear_extra_traces(&mut list, 6);
        let mut actual: Vec<TraceItem> = Vec::new();
        for i in list.iter(){
            actual.push(i.clone());
        }

        let expected = vec![
            TraceItem {
                pos: Vec2::ZERO,
                index: 9,
            },
            TraceItem {
                pos: Vec2::ZERO,
                index: 8,
            },
            TraceItem {
                pos: Vec2::ZERO,
                index: 7,
            },
            TraceItem {
                pos: Vec2::ZERO,
                index: 6,
            }
        ];
        assert_eq!(actual, expected)
    }
}