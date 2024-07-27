use bevy::color::palettes::basic::YELLOW;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Gizmos, KeyCode, Query, Res};
use crate::Snake;

pub fn snake(
    commands: Commands,
    mut gizmos: Gizmos,
    mut snake_query: Query<&mut Snake>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    for mut snake in &mut snake_query {
        let head_radius = 50.0;

        let update = {
            let mut result = Vec2::new(0.0, 0.0);
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                result += Vec2::new(10.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                result += Vec2::new(-10.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                result += Vec2::new(0.0, 10.0);
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                result += Vec2::new(0.0, -10.0);
            }
            result
        };
        snake.head_pos += update;

        gizmos.circle_2d(snake.head_pos, head_radius, YELLOW);
    }
}