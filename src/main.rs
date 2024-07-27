mod grid;
mod sprite;

use bevy::{
    prelude::*,
    sprite:: {
        Wireframe2dPlugin
    }
};
use std::f32;
use bevy::color::palettes::css::*;
use bevy::input::keyboard::keyboard_input_system;
use crate::grid::*;
use crate::sprite::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_grid)
        .add_systems(Update, sprite_movement)
        .add_systems(Update, snake)
        .run();
}

enum Direction {
    Up,
    Down,
}

#[derive(Component)]
struct Sprite {
    position: Vec2,
    direction: Direction
}

#[derive(Component)]
struct Snake {
    name: String,
    head_pos: Vec2,
    head_direction_angle: f32,
    distance_from_last_turn: f32,
    direction_changes: Vec<DirectionChange>,
    //linear speed in meters per second
    movement_speed: f32,
    //rotation speed in radians per second
    rotation_speed: f32
}
#[derive(Component)]
struct DirectionChange {
    old_direction_angle: f32,
    distance_from_last_turn: f32
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..3
    {
        commands.spawn(Sprite {
            position: Vec2::new(i as f32 * 100.0 - 400., -100.0),
            direction: Direction::Up,
        });
    }

    for i in 0..2 {
        commands.spawn(Snake {
            name: format!("snake #{i}"),
            head_pos: Vec2::new(i as f32 * 20.0, 0.0),
            head_direction_angle: 0.0,
            distance_from_last_turn: 0.0,
            direction_changes: vec![],
            movement_speed: 0.0,
            rotation_speed: 0.0,
        });
    }
}
fn snake(
    commands: Commands,
    mut gizmos: Gizmos,
    mut snake_query: Query<&mut Snake>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    for mut snake in &mut snake_query {
        let head_radius = 50.0;

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            snake.head_pos += Vec2::new(10.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            snake.head_pos += Vec2::new(-10.0, 0.0);
        }
        gizmos.circle_2d(snake.head_pos, head_radius, YELLOW);
        //snake.head_pos += Vec2::new(0.0, 10.0);
    }
}