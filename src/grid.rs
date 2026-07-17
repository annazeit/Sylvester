
use bevy::prelude::*;
use bevy::{
    color::palettes::basic::*,
};
use std::f32;
use bevy::app::{App, Plugin};
use bevy::color::palettes::css::GREY;

pub struct VisualDiagnosticPlugin;

// Toggled by pressing "1" (see draw_grid below). When enabled, draws a debug grid
// and switches on gizmo drawing for hitboxes/paths in other systems (snake, food, bound).
#[derive(Component)]
pub struct GridVisualDiagnostic {
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
    commands.spawn(GridVisualDiagnostic{
        enabled: false,
        cell_size: 50.0
    });
}

/// Visual diagnostics enabled when we need to draw extra entity shapes using gizmos to help to debug game.
/// Without visual diagnostics gizmos will not be used and we will see only SpriteBundles.
/// Drawing gizmos with SpriteBundles together helps to test the game.
pub fn grid_draw_visual_diagnostics_info(query: &Query<&GridVisualDiagnostic>) -> bool {
    for item in query {
        if item.enabled { 
            return true;
        }
    }
    return false;
}

// Half-width/height the grid lines span - how far out the debug grid reaches.
const GRID_EXTENT: f32 = 5000.0;

fn draw_grid(
    mut gizmos: Gizmos,
    mut grid_query: Query<&mut GridVisualDiagnostic>,
    keyboard_input: Res<ButtonInput<KeyCode>>
){
    for mut grid in &mut grid_query {
        if keyboard_input.just_pressed(KeyCode::Digit1) {
            grid.enabled = !grid.enabled;
        }

        if grid.enabled {
            let line_count = (2.0 * GRID_EXTENT / grid.cell_size) as i32;
            for i in 1..line_count {
                let start_pos: Vec2 = Vec2::new(-GRID_EXTENT, -GRID_EXTENT + (i as f32 * grid.cell_size));
                let end_pos: Vec2 = Vec2::new(GRID_EXTENT, -GRID_EXTENT + (i as f32 * grid.cell_size));
                gizmos.line_2d(start_pos, end_pos, GREY);
            }
            for i in 1..line_count {
                let start_pos: Vec2 = Vec2::new(-GRID_EXTENT + (i as f32 * grid.cell_size), -GRID_EXTENT);
                let end_pos: Vec2 = Vec2::new(-GRID_EXTENT + (i as f32 * grid.cell_size), GRID_EXTENT);
                gizmos.line_2d(start_pos, end_pos, GREY);
            }
            gizmos.line_2d(Vec2::new(-5.0, -5.0), Vec2::new(5.0, 5.0), RED);
            gizmos.line_2d(Vec2::new(5.0, -5.0), Vec2::new(-5.0, 5.0), RED);
        }
    }
}
