
use bevy::prelude::*;
use bevy::{
    color::palettes::basic::*,
    prelude::*,
};
use std::f32;
use bevy::color::palettes::css::GREY;

pub fn draw_grid(mut gizmos: Gizmos) {
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
