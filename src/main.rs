mod grid;
mod sprite;
mod snake;

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
        .add_plugins(crate::snake::SnakePlugin)
        //.add_plugins(crate::sprite::SpritePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_grid)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

}