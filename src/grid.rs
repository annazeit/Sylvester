
use bevy::prelude::*;
use bevy::{
    color::palettes::basic::*,
};
use std::f32;
use bevy::app::{App, Plugin};
use bevy::color::palettes::css::GREY;

pub struct VisualDiagnosticPlugin;

#[derive(Component)]
pub struct VisualDiagnostic {
    enabled: bool,
    cell_size: f32,
}

impl Plugin for VisualDiagnosticPlugin{
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, grid_start);
        app.add_systems(Update, draw_grid);
    }
}

fn grid_start (mut commands: Commands) {
    commands.spawn(VisualDiagnostic{
        enabled: false,
        cell_size: 50.0
    });
}

/// Visual diagnostics enabled when we need to draw extra entity shapes using gizmos to help to debug game.
/// Without visual diagnostics gizmos will not be used and we will see only SpriteBundles.
/// Drawing gizmos with SpriteBundles together helps to test the game.
pub fn draw_visual_diagnostics_info(query: &Query<&VisualDiagnostic>) -> bool {
    for item in query {
        if item.enabled { 
            return true;
        }
    }
    return false;
}

fn draw_grid(
    mut gizmos: Gizmos,
    mut grid_query: Query<&mut VisualDiagnostic>,
    keyboard_input: Res<ButtonInput<KeyCode>>
){
    for mut grid in &mut grid_query {
        if keyboard_input.just_pressed(KeyCode::Digit1) { 
            grid.enabled = !grid.enabled;    
        }

        if grid.enabled {
            for i in 1..50 {
                let start_pos: Vec2 = Vec2::new(-1000.0, -1000.0 + (i as f32 * grid.cell_size));
                let end_pos: Vec2 = Vec2::new(1000.0, -1000.0 + (i as f32 * grid.cell_size));
                gizmos.line_2d(start_pos, end_pos, GREY);
            }
            for i in 1..50 {
                let start_pos: Vec2 = Vec2::new(-1000.0 + (i as f32 * grid.cell_size), -1000.0);
                let end_pos: Vec2 = Vec2::new(-1000.0 + (i as f32 * grid.cell_size), 1000.0);
                gizmos.line_2d(start_pos, end_pos, GREY);
            }
            gizmos.line_2d(Vec2::new(-5.0, -5.0), Vec2::new(5.0, 5.0), RED);
            gizmos.line_2d(Vec2::new(5.0, -5.0), Vec2::new(-5.0, 5.0), RED);
        }   
    }
}
