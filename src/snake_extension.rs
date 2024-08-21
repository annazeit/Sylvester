use bevy::ecs::query;
use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::math::Vec2;
use std::f32::*;

use bevy::color::palettes::css::*;
use bevy::input::ButtonInput;
use bevy::sprite::SpriteBundle;

use crate::grid::*;
use crate::snake_model::*;
use crate::trace_position_calculator::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, snake_start);
        app.add_systems(Update, snake_update);
    }
}

/// All creature visual movable parts will have this component to query their transformations.  
#[derive(Component)]
struct  CreatureBodyVisualElement;

fn snake_start (mut commands: Commands,  asset_server: Res<AssetServer>) {
    for mut snake_model in snake_head_new_list() {
        let head_entity = commands.spawn((
            SpriteBundle {
                texture: asset_server.load("SnakeHead.png"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 1.0, 1.0)),
                ..default()
            },
            CreatureBodyVisualElement
        )).id();

        snake_model.body = BodyType::BasicHeadOnly(head_entity);
        commands.spawn(snake_model);
    }
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

fn draw_circle(gizmos: &mut Gizmos, position: Vec2, radius: f32, query: &Query<&VisualDiagnostic>) {
    if draw_visual_diagnostics_info(&query) {
        gizmos.circle_2d(position, radius, YELLOW);
    }
}

fn draw_tail(gizmos: &mut Gizmos, radius: f32, snake: &SnakeModel, query: &Query<&VisualDiagnostic>){
    let mut distance = radius * 2.0;
    for i in 1..=2 {
        let shift_from_head: Vec2 = {
            let x_tail = f32::sin(snake.head_direction_angle - consts::PI) * distance;
            let y_tail = f32::cos(snake.head_direction_angle - consts::PI) * distance;
            Vec2::new(x_tail, y_tail)
        };

        let tail_pos = snake.head_pos + shift_from_head;
        let tail_radius = radius - (20.0 * i as f32);
        distance += 75.0;
        draw_circle(gizmos, tail_pos, tail_radius, &query);
    }
}

fn get_last_trace_index_before_clean(snake: &SnakeModel, gizmos: &mut Gizmos) -> i64 {
    let mut current_pos = snake.head_pos;
    let mut total_distance = 0.0;

    let step = snake.tracing_step;
    let mut color_change = 0;
    let mut last_trace_index_before_clean = 0;
    for i in snake.trace.iter() {
        total_distance += current_pos.distance(i.pos);
        let color = Color::hsl(360.0 * color_change as f32 / step as f32, 0.95, 0.7);
        color_change += 1;

        gizmos.line_2d(current_pos, i.pos, color);
        current_pos = i.pos;
        if total_distance > 20.0 + snake.size * snake.node_radius * 2.0 {
            last_trace_index_before_clean = i.index;
            break;
        }
    }
    return last_trace_index_before_clean;
}

fn draw_nodes(snake: &mut SnakeModel, gizmos: &mut Gizmos,) {
    for i in 0..(snake.size) as i32 {
        let distance_from_head = i as f32 * (snake.tracing_step * 2.0);
        let mut trace_positions_iterator = snake.trace.iter().map(|p| p.pos);
        let node_pos = calculate_node_pos_traced_on_distance_from_head(snake.head_pos, &mut trace_positions_iterator, snake.trace.len(), distance_from_head);
        gizmos.circle_2d(node_pos, snake.node_radius, WHITE);
    }
}

fn snake_update (
    mut gizmos: Gizmos, 
    mut snake_query: Query<&mut SnakeModel>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    query: Query<&VisualDiagnostic>,
    mut query_visual_element: Query<&mut Transform, With<CreatureBodyVisualElement>>,
    mut commands: Commands
) {
    for mut snake in &mut snake_query {
        snake.head_direction_angle += keyboard_rotation(&keyboard_input, &snake, &time) * (snake.movement_speed / 4.0);

        let keyboard_up_down_input = keyboard_movement_up_down_impure(&keyboard_input);
        head_move_pure(keyboard_up_down_input, time.delta_seconds(), &mut snake);
        
        let last_trace_index_before_clean = get_last_trace_index_before_clean(&snake, &mut gizmos);
        clear_extra_traces(&mut snake.trace, last_trace_index_before_clean);

        draw_circle(&mut gizmos, snake.head_pos, snake.head_radius, &query); // draws snake head

        draw_tail(&mut gizmos, snake.head_radius, &snake, &query);

        draw_nodes(&mut snake, &mut gizmos);

        match &snake.body {
            BodyType::BasicHeadOnly(head_entity) => {
                let mut t = query_visual_element.get_mut(*head_entity).unwrap();
                t.translation = Vec3::new(snake.head_pos.x, snake.head_pos.y, 0.0);
                t.rotation = Quat::from_rotation_z(-snake.head_direction_angle);
            },
            BodyType::Snake(_) => {

            },
            BodyType::JellyFish => {

            }
        };
    }
}