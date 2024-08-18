mod grid;
mod sprite;
mod snake_extension;
mod snake_model;
mod food;
mod snake_model_tests;
mod creature_body_evolution;

use bevy::{
    prelude::*,
    sprite::Wireframe2dPlugin,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_plugins(crate::grid::VisualDiagnosticPlugin)
        .add_plugins(crate::snake_extension::SnakePlugin)
        .add_plugins(crate::food::FoodPlugin)
        //.add_plugins(crate::sprite::SpritePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let sprite_handle = asset_server.load("Test.png");
    commands.spawn(SpriteBundle {
        texture: sprite_handle.clone(),
        ..default()
    });
}