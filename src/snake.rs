use bevy::color::palettes::basic::YELLOW;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Gizmos, KeyCode, Query, Res};
use crate::Snake;

pub fn snake(
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