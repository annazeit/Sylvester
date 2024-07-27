mod grid;
mod sprite;
mod snake;

use bevy::{
    prelude::*,
    sprite:: {
        Wireframe2dPlugin
    }
};
use std::f32;
use crate::grid::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_plugins(crate::snake::SnakePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_grid)
        .add_systems(Update, crate::sprite::sprite_movement)
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..3
    {
        commands.spawn(Sprite {
            position: Vec2::new(i as f32 * 100.0 - 400., -100.0),
            direction: Direction::Up,
        });
    }


}