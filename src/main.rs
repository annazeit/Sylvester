//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;
use bevy::{
    color::palettes::basic::*,
    prelude::*,
    sprite:: {
        Wireframe2dPlugin
    }
};
use std::f32;
use bevy::color::palettes::css::GREY;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .add_systems(Update, draw_grid)
        .run();
}

enum Direction {
    Up,
    Down,
}

#[derive(Component)]
struct Player {
    position: Vec2,
    direction: Direction
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    for i in 0..3
    {
        commands.spawn(Player {
            position: Vec2::new(i as f32 * 100.0 - 400., -100.0),
            direction: Direction::Up,
        });
    }
}

fn draw_grid(mut gizmos: Gizmos) {
    let cell_size = 100.0;
    for i in 1..30 {
        let start_pos: Vec2 = Vec2::new(-1000.0, -1000.0 + (i as f32 * cell_size));
        let end_pos: Vec2 = Vec2::new(1000.0, -1000.0 + (i as f32 * cell_size));
        gizmos.line_2d(start_pos, end_pos, GREY);
    }
    for i in 1..30 {
        let start_pos: Vec2 = Vec2::new(-1000.0 + (i as f32 * cell_size), -1000.0);
        let end_pos: Vec2 = Vec2::new(-1000.0 + (i as f32 * cell_size), 1000.0);
        gizmos.line_2d(start_pos, end_pos, GREY);
    }
    gizmos.line_2d(Vec2::new(-5.0, -5.0), Vec2::new(5.0, 5.0), RED);
    gizmos.line_2d(Vec2::new(5.0, -5.0), Vec2::new(-5.0, 5.0), RED);
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<&mut Player>, mut gizmos: Gizmos) {

    for mut player in &mut sprite_position {
        player.position = {
            let y_move = {
                match player.direction {
                    Direction::Up => 150. * time.delta_seconds(),
                    Direction::Down => -150. * time.delta_seconds(),
                }
            };
            player.position + Vec2::new(0.0, y_move)
        };

        if player.position.y > 200. {
            player.direction = Direction::Down;
        } else if player.position.y < -200. {
            player.direction = Direction::Up;
        }

        let steps = 30;
        for i in 0..steps {
            let position: Vec2 = {
                let total_length = 400.0;
                let step_length = total_length / steps as f32;
                let x = {

                    let wiggle_speed = 60.0;
                    let radian_in_sec = 2.0 * f32::consts::PI / 60.0;

                    let time_angle = time.elapsed_seconds() * radian_in_sec * wiggle_speed;
                    let step_angle = 2.0 * f32::consts::PI * (i as f32 / steps as f32);
                    // from -1 to 1
                    let seconds_cycle = f32::sin(time_angle + step_angle);
                    let wave_amplitude = 20.0;
                    wave_amplitude * seconds_cycle
                };
                player.position + Vec2::new(x, (i as f32) * step_length)
            };

            let radius = {
                let radius_factor = f32::sin(2.0 * f32::consts::PI * (i as f32 / steps as f32));
                (radius_factor) * 30.0 + 10.0
            };

            let color = Color::hsl(360. * i as f32 / steps as f32, 0.95, 0.7);
            gizmos.circle_2d(position, radius, color);
        }
    }
}
