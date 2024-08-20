use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Component, Entity, Res};
use bevy::sprite::SpriteBundle;
use std::collections::LinkedList;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct TraceItem {
    pub pos: Vec2,
    pub index: i64,
}

pub enum SnakeSpireNodeType {
    Big(Entity),
    Medium,
    Small,
}

pub struct SnakeSpineNode{
    pub distance_from_head: f32,
    pub node_type: SnakeSpireNodeType
}

pub enum BodyType {
    Snake(Vec<SnakeSpineNode>),
    JellyFish
}

#[derive(Component)]
pub struct SnakeModel {
    pub head_pos: Vec2,
    pub head_direction_angle: f32,
    pub head_radius: f32,//?
    // linear speed in meters per second
    pub movement_speed: f32,
    // rotation speed in degrees per second. this value defines how quickly the object changes direction
    pub rotation_speed_in_degrees: f32,
    // increases every time that a new TraceItem is added to the LinkedList, it is used as the index of the last segment
    pub trace_counter: i64,
    pub trace: LinkedList<TraceItem>,
    pub tracing_step: f32,
    // NOT number of foods eaten by snake // number of nodes drawn
    pub size: f32, 
    pub node_radius: f32,
    pub body: BodyType
}

pub enum  SnakeMoveDirection {
    Forward,
    Backward,
    Stop
}

pub fn snake_model_new(i: i32) -> SnakeModel {
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
        size: 5.0,
        node_radius: 10.0,
        body: BodyType::Snake(vec![])
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

pub fn head_move_pure(keyboard_up_down_input: SnakeMoveDirection, time_delta_seconds: f32, snake: &mut SnakeModel) {
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
    if distance_between >= snake.tracing_step {
        let point = snake.head_pos + new_head_move;
        snake.trace_counter += 1;
            let trace_item = TraceItem {
                pos: point,
                index: snake.trace_counter
            };
            snake.trace.push_front(trace_item);
    }

    snake.head_pos += new_head_move;

}
