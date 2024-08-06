mod grid;
mod sprite;
mod snake_extension;
mod snake_model;
mod food;

use bevy::{
    prelude::*,
    sprite:: {
        Wireframe2dPlugin
    }
};
use crate::grid::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_plugins(crate::snake_extension::SnakePlugin)
        .add_plugins(crate::food::FoodPlugin)
        //.add_plugins(crate::sprite::SpritePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_grid)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

}