use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::math::{Vec2, VectorSpace};
use consts::PI;
use std::f32::*;

use bevy::color::palettes::css::*;
use bevy::input::ButtonInput;

use crate::creature_body_evolution::*;
use crate::foo::*;
use crate::grid::*;
use crate::snake_model::*;
use crate::trace_position_calculator::*;
use crate::start::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, snake_start);
        app.add_systems(Update, snake_update);
    }
}


fn snake_start (mut commands: Commands,  asset_server: Res<AssetServer>) {
    for mut snake in snake_head_new_list() {
        let list = spine_from_size(&mut commands, &asset_server, &mut snake);
        snake.body = list;
        commands.spawn(snake);
    }
}

fn keyboard_movement_up_down_impure(keyboard_input: &Res<ButtonInput<KeyCode>>) -> SnakeMoveDirection {
    if keyboard_input.pressed(KeyCode::ArrowUp) { SnakeMoveDirection::Forward }
    else if keyboard_input.pressed(KeyCode::ArrowDown) { SnakeMoveDirection::Backward }
    else { SnakeMoveDirection::Stop }
}

fn keyboard_rotation(keyboard_input: &Res<ButtonInput<KeyCode>>, snake: &SnakeModel, time: &Res<Time>) -> f32 {
    let unit: f32 = {
        if keyboard_input.pressed(KeyCode::ArrowRight) { -1.0 }
        else if keyboard_input.pressed(KeyCode::ArrowLeft) { 1.0 }
        else { 0.0 }
    };
    consts::PI / 180.0 * snake.rotation_speed_in_degrees * unit * time.delta_seconds()
}

fn draw_circle(gizmos: &mut Gizmos, position: Vec2, radius: f32, grid_query: &Query<&GridVisualDiagnostic>) {
    if grid_draw_visual_diagnostics_info(&grid_query) {
        gizmos.circle_2d(position, radius, YELLOW);
    }
}

fn draw_tail(gizmos: &mut Gizmos, radius: f32, snake: &SnakeModel, grid_query: &Query<&GridVisualDiagnostic>){
    let mut distance = radius * 2.0;
    for i in 1..=2 {
        let shift_from_head: Vec2 = {
            let x_tail = f32::cos(snake.head_direction_angle - consts::PI) * distance;
            let y_tail = f32::sin(snake.head_direction_angle - consts::PI) * distance;
            Vec2::new(x_tail, y_tail)
        };

        let tail_pos = snake.head_pos + shift_from_head;
        let tail_radius = radius - (20.0 * i as f32);
        distance += 75.0;
        draw_circle(gizmos, tail_pos, tail_radius, &grid_query);
    }
}

fn get_last_trace_index_before_clean(snake: &SnakeModel, gizmos: &mut Gizmos) -> i64 {
    let mut current_pos = snake.head_pos;
    let mut total_distance = 0.0;

    let mut last_trace_index_before_clean = 0;
    for i in snake.trace.iter() {
        total_distance += current_pos.distance(i.pos);
        current_pos = i.pos;
        
        if total_distance > 20.0 + snake.size * snake.node_radius * 2.0 {
            last_trace_index_before_clean = i.index;
            break;
        }
    }
    return last_trace_index_before_clean;
}

fn draw_nodes(snake: &mut SnakeModel, gizmos: &mut Gizmos, query_visual_element: &mut Query<&mut Transform, With<CreatureBodyVisualElement>>, start_button_query: &Query<&StartVisualDiagnostic>,) {
    let mut current_pos = snake.head_pos;
    let step = snake.tracing_step;
    let mut color_change = 0;

    if start_button_draw_visual_diagnostics_info(&start_button_query) {
        println!("AAAAAAAAAAAAA");
    }
    
    for i in 0..=(snake.size) as i32 {
        let distance_from_head = i as f32 * (snake.tracing_step * 2.0);
        let trace_positions_iterator = snake.trace.iter().map(|p| p.pos);
        let node_calc_result = calculate_node_pos_traced_on_distance_from_head(
            snake.head_pos, 
            snake.head_direction_angle,
            trace_positions_iterator, 
            distance_from_head
        );
        
        //gizmos.circle_2d(node_pos, snake.node_radius, BLUE);
        
        let color = Color::hsl(360.0 * color_change as f32 / step as f32, 0.95, 0.7);
        color_change += 1; 

        gizmos.line_2d(current_pos, node_calc_result.position, color);
        if i != 0 {
            current_pos = node_calc_result.position;
        }

        let snake_node = {
            let mut node: Mut<Transform> = query_visual_element.get_mut(snake.body[i as usize].node_type).unwrap();
            node.translation = Vec3::new(node_calc_result.position.x, node_calc_result.position.y, 0.0); 

            //println!("{:?}", node_calc_result.directions.segment_distance_fraction.to_string());
            let a = interpolate_direction(
                node_calc_result.directions.direction_previous, 
                node_calc_result.directions.direction_current,
                node_calc_result.directions.direction_next,
                node_calc_result.directions.segment_distance_fraction,
            );
            node.rotation = Quat::from_rotation_z(a + PI / 2.0 + PI);
        };
    }
}

fn snake_update (
    mut gizmos: Gizmos, 
    mut snake_query: Query<&mut SnakeModel>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    grid_query: Query<&GridVisualDiagnostic>,
    start_button_query: Query<&StartVisualDiagnostic>,
    mut query_visual_element: Query<&mut Transform, With<CreatureBodyVisualElement>>,
) {
    for mut snake in &mut snake_query {
        // if start_button_draw_visual_diagnostics_info(&start_button_query) {
        //     gizmos.circle_2d(Vec2::new(0.0, 200.0), 200.0, RED);
        //     println!("AAAAAAAAAAAAA");
        // }
        snake.head_direction_angle += keyboard_rotation(&keyboard_input, &snake, &time) * (snake.movement_speed / 4.0);

        let keyboard_up_down_input: SnakeMoveDirection = keyboard_movement_up_down_impure(&keyboard_input);
        head_move_pure(keyboard_up_down_input, time.delta_seconds(), &mut snake);

        let node_pos = draw_nodes(&mut snake, &mut gizmos, &mut query_visual_element, &start_button_query);
        
        let last_trace_index_before_clean = get_last_trace_index_before_clean(&snake, &mut gizmos);
        clear_extra_traces(&mut snake.trace, last_trace_index_before_clean);

        draw_circle(&mut gizmos, snake.head_pos, snake.head_radius, &grid_query); // draws hidden snake head in gizmos

        draw_tail(&mut gizmos, snake.head_radius, &snake, &grid_query);

        draw_nodes(&mut snake, &mut gizmos, &mut query_visual_element, &start_button_query);

        let snake_head = {
            let mut head: Mut<Transform> = query_visual_element.get_mut(snake.body[0].node_type).unwrap();
            head.translation = Vec3::new(snake.head_pos.x, snake.head_pos.y, 0.0); 
            head.rotation = Quat::from_rotation_z(snake.head_direction_angle + PI / 2.0 + PI);
        };  
    }
} 