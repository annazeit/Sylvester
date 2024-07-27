use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::palettes::basic::YELLOW;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Gizmos, KeyCode, Query, Res};

pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, snake_start);
        app.add_systems(Update, snake_update);
    }
}
fn snake_start(mut commands: Commands) {
    for i in 0..1 {
        commands.spawn(Snake {
            name: format!("snake #{i}"),
            head_pos: Vec2::new(i as f32 * 20.0, 0.0),
            head_direction_angle: 0.0,
            distance_from_last_turn: 0.0,
            direction_changes: vec![],
            movement_speed: 5.0,
            rotation_speed_in_degrees: 3.0,
        });
    }}

#[derive(Component)]
struct Snake {
    name: String,
    head_pos: Vec2,
    head_direction_angle: f32,
    distance_from_last_turn: f32,
    direction_changes: Vec<DirectionChange>,
    //linear speed in meters per second
    movement_speed: f32,
    //rotation speed in degrees per second. this value defines how quickly the object changes direction
    rotation_speed_in_degrees: f32
}
#[derive(Component)]
struct DirectionChange {
    old_direction_angle: f32,
    distance_from_last_turn: f32
}

fn snake_update(
    commands: Commands,
    mut gizmos: Gizmos,
    mut snake_query: Query<&mut Snake>,
    mut keyboard_input: Res<ButtonInput<KeyCode>>
) {
    for mut snake in &mut snake_query {
        let head_radius = 50.0;

        let movement: f32 = {
            let unit = {
                if keyboard_input.pressed(KeyCode::ArrowUp) { 1.0 }
                else if keyboard_input.pressed(KeyCode::ArrowDown) { -1.0 }
                else { 0.0 }
            };
            unit * snake.movement_speed
        };
        let rotation: f32 = {
            if keyboard_input.pressed(KeyCode::ArrowRight) { 1.0 }
            else if keyboard_input.pressed(KeyCode::ArrowLeft) { -1.0 }
            else { 0.0 }
        };

        snake.head_direction_angle += std::f32::consts::PI / 180.0 * snake.rotation_speed_in_degrees * rotation;
        let x = f32::sin(snake.head_direction_angle) * movement;
        let y = f32::cos(snake.head_direction_angle) * movement;
        snake.head_pos += Vec2::new(x, y);

        gizmos.circle_2d(snake.head_pos, head_radius, YELLOW);
    }
}