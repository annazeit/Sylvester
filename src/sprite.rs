use std::f32;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Gizmos, Mut, Query, Res, Time};
use crate::{Direction, Sprite};

fn sprite_update(sprite: &mut Mut<Sprite>, time: &Res<Time>){
    sprite.position = {
        let y_move = {
            match sprite.direction {
                Direction::Up => 150. * time.delta_seconds(),
                Direction::Down => -150. * time.delta_seconds(),
            }
        };
        sprite.position + Vec2::new(0.0, y_move)
    };

    if sprite.position.y > 200. {
        sprite.direction = Direction::Down;
    } else if sprite.position.y < -200. {
        sprite.direction = Direction::Up;
    }
}
fn sprite_animate(sprite: &mut Mut<Sprite>, time: &Res<Time>, gizmos: &mut Gizmos) {
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
            sprite.position + Vec2::new(x, (i as f32) * step_length)
        };

        let radius = {
            let radius_factor = f32::sin(2.0 * f32::consts::PI * (i as f32 / steps as f32));
            (radius_factor) * 30.0 + 10.0
        };

        let color = Color::hsl(360. * i as f32 / steps as f32, 0.95, 0.7);
        gizmos.circle_2d(position, radius, color);
    }
}

// The sprite is animated by changing its translation depending on the time that has passed since the last frame.
pub fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<&mut Sprite>,
    mut gizmos: Gizmos,
) {
    for mut sprite in &mut sprite_position {
        sprite_update(&mut sprite, &time);
        sprite_animate(&mut sprite, &time, &mut gizmos);
    }
}
