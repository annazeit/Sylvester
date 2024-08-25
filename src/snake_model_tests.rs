use bevy::prelude::Vec2;
use std::collections::LinkedList;
use crate::snake_model::*;

#[cfg(test)]
mod tests {
    use std::f32::consts::{self, PI};

    use super::*;

    fn assert_vec2_eq(a: Vec2, b: Vec2) {
        assert_float_eq(a.x, b.x);
        assert_float_eq(a.y, b.y)
    }
    fn assert_float_eq(a: f32, b: f32) {
        let delta_max = 0.001;
        let c = f32::abs(a - b);
        assert!(c < delta_max)
    }

    #[test]
    fn no_move_because_not_key_input() {
        let mut snake = snake_model_new(0);
        let traces_original = snake.trace.clone();
        snake.tracing_step = 50.0;
        head_move_pure(SnakeMoveDirection::Stop, 10.0, &mut snake);
        let expected_move = Vec2::new(0.0, 0.0);

        assert_vec2_eq(snake.head_pos, expected_move);
        // trace without changes
        assert_eq!(snake.trace, traces_original);
    }

    #[test]
    fn move_forward_north() {
        let mut snake = snake_model_new(0);
        let traces_original = snake.trace.clone();
        snake.tracing_step = 50.0;
        snake.head_direction_angle = PI / 2.0;
        snake.movement_speed = 3.0;
        head_move_pure(crate::snake_model::SnakeMoveDirection::Forward, 10.0, &mut snake);
        let expected_move = Vec2::new(0.0, 30.0);

        assert_vec2_eq(snake.head_pos, expected_move);
        assert_eq!(snake.trace, traces_original);
    }

    #[test]
    fn move_backward_south() {
        let mut snake = snake_model_new(0);
        let traces_original = snake.trace.clone();
        snake.tracing_step = 50.0;
        snake.movement_speed = 3.0;
        head_move_pure(crate::snake_model::SnakeMoveDirection::Backward, 10.0, &mut snake);
        let expected_move = Vec2::new(0.0, -30.0);

        assert_vec2_eq(snake.head_pos, expected_move);
        assert_eq!(snake.trace, traces_original);
    }

    #[test]
    fn move_forward_south() {
        let mut snake = snake_model_new(0);
        let traces_original = snake.trace.clone();
        snake.tracing_step = 50.0;
        snake.head_direction_angle = -PI / 2.0;
        snake.movement_speed = 3.0;
        head_move_pure(crate::snake_model::SnakeMoveDirection::Forward, 10.0, &mut snake);
        let expected_move = Vec2::new(0.0, -30.0);

        assert_vec2_eq(snake.head_pos, expected_move);
        assert_eq!(snake.trace, traces_original);
    }

    #[test]
    fn move_backward_north() {
        let mut snake = snake_model_new(0);
        let traces_original = snake.trace.clone();
        snake.tracing_step = 50.0;
        snake.head_direction_angle = -PI / 2.0;
        snake.movement_speed = 3.0;
        head_move_pure(crate::snake_model::SnakeMoveDirection::Backward, 10.0, &mut snake);
        let expected_move = Vec2::new(0.0, 30.0);

        assert_vec2_eq(snake.head_pos, expected_move);
        assert_eq!(snake.trace, traces_original);
    }

    #[test]
    fn trace_track_move_up() {
        let mut snake = snake_model_new(0);
        let mut traces_expected = snake.trace.clone();
        snake.tracing_step = 50.0;
        snake.head_direction_angle = PI / 2.0;
        snake.movement_speed = 5.0;
        head_move_pure(crate::snake_model::SnakeMoveDirection::Forward, 10.0, &mut snake);
        let expected_move = Vec2::new(0.0, 50.0);

        assert_vec2_eq(snake.head_pos, expected_move);

        traces_expected.push_front(TraceItem{
            pos: expected_move,
            index: 1
        });
        let traces_expected_vect: Vec<TraceItem> = traces_expected.clone().into_iter().collect();
        let trace_actual: Vec<TraceItem> = snake.trace.clone().into_iter().collect();
        let mut trace_zip_iter = traces_expected_vect.iter().zip(trace_actual.iter());

        for (expected, actual) in trace_zip_iter {
            assert_eq!(expected.index, actual.index);
            assert_vec2_eq(expected.pos, actual.pos);
        }
    }

    #[test]
    fn trace_track_move_up_with_diff_headpos() {
        let mut snake = snake_model_new(0);
        let mut traces_expected = snake.trace.clone();
        snake.tracing_step = 50.0;
        snake.head_pos = Vec2::new(0.0, 100.0);
        snake.head_direction_angle = PI / 2.0;
        snake.movement_speed = 5.0;
        head_move_pure(crate::snake_model::SnakeMoveDirection::Forward, 10.0, &mut snake);
        let expected_move = Vec2::new(0.0, 150.0);

        assert_vec2_eq(snake.head_pos, expected_move);

        traces_expected.push_front(TraceItem{
            pos: expected_move,
            index: 1
        });
        let traces_expected_vect: Vec<TraceItem> = traces_expected.clone().into_iter().collect();
        let trace_actual: Vec<TraceItem> = snake.trace.clone().into_iter().collect();
        let mut trace_zip_iter = traces_expected_vect.iter().zip(trace_actual.iter());

        for (expected, actual) in trace_zip_iter {
            assert_eq!(expected.index, actual.index);
            assert_vec2_eq(expected.pos, actual.pos);
        }
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