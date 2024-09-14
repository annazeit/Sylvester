use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::palettes::basic::RED;
use bevy::color::{Color, Srgba};
use bevy::math::Vec2;
use bevy::prelude::*;
use rand::Rng;
use std::f32::*;

use crate::snake_model::SnakeModel;
use crate::grid::*;

pub struct FoodPlugin;

#[derive(Component)]
pub struct Food {
    pos: Vec2,
    direction: f32,
    radius: f32,
    color: Srgba,
}
#[derive(Component)]
pub struct Bound {
    pos: Vec2,
    radius: f32,
}

#[derive(Component)]
struct Score {
    score_num: i32
}

impl Plugin for FoodPlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, food_start);
        app.add_systems(Startup, score_start);
        app.add_systems(Startup, bound_start);
        app.add_systems(Update, food_update);
        app.add_systems(Update, draw_bound);
    }
}

fn bound_start(mut commands: Commands) {
    commands.spawn(Bound{
        pos: Vec2::new(0.0, 0.0),
        radius: 500.0,
    });
}
fn score_start(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("MovistarTextRegular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    let text_justification = JustifyText::Center;

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("no score", text_style.clone())
                .with_justify(text_justification),
            transform: Transform::from_xyz(0.0, 200.0, 0.0),
            ..default()
        },
        Score { score_num: 0 }
    ));
}
fn food_start (mut commands: Commands, asset_server: Res<AssetServer>) {
    let food_image_size = 100.0;
    let radius = 10.0;
    let scale = (radius * 2.0) / food_image_size;
    for _ in 0..5 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("Food.png"),
                transform: Transform::from_xyz(120.0, 0.0, 0.0).with_scale(Vec3::new(scale, scale, scale)),
                ..default()
            },
            Food {
                pos: new_food_position(),
                direction: new_food_direction(),
                radius,
                color: new_food_color(),
            }
        ));
    }
}

fn draw_bound(
    mut gizmos: Gizmos,
    bound_query: Query<&mut Bound>,
    query: Query<&VisualDiagnostic>
) {
    for bound in &bound_query{
        if draw_visual_diagnostics_info(&query) {
            gizmos.circle_2d(bound.pos, bound.radius, RED);
        }
    }
}

fn new_food_position() -> Vec2 {
    let x = rand::thread_rng().gen_range(-300..=300) as f32;
    let y = rand::thread_rng().gen_range(-300..=300) as f32;
    Vec2::new(x, y)
}
fn new_food_direction() -> f32 {
    let num = rand::thread_rng().gen_range(0.0..= consts::PI * 2.0) as f32;
    num
}
fn new_food_color() -> Srgba {
    let mut rnd = rand::thread_rng();
    let hue: f32 = rnd.gen();
    let color: Srgba = Color::hsl(hue * 360.0, 0.95, 0.7).to_srgba();
    color
}

fn snake_eats_food(
    snake: &SnakeModel,
    food: &Food
) -> bool {
    let distance_vector = snake.head_pos - food.pos;
    let distance_between = ((distance_vector.x * distance_vector.x) + (distance_vector.y * distance_vector.y)).sqrt();
    distance_between < food.radius + snake.head_radius
}
fn food_on_bound(food: &mut Food, bound_query: &Query<&mut Bound> ) {
    for bound in bound_query {
        let origin = Vec2::new(0.0, 0.0);
        let distance_from_origin_to_food: f32 = {
            let distance_vector = origin - food.pos;
            ((distance_vector.x * distance_vector.x) + (distance_vector.y * distance_vector.y)).sqrt()
        };
        if distance_from_origin_to_food > (bound.radius - (food.radius * 2.0 )) {
            food.direction = new_food_direction()
        }
    }
}

fn draw_food(food: &mut Food, gizmos: &mut Gizmos, query: &Query<&VisualDiagnostic>) {
    let food_move = {
        let x = f32::cos(food.direction);
        let y = f32::sin(food.direction);
        Vec2::new(x, y)
    };
    food.pos += food_move;
    
    if draw_visual_diagnostics_info(&query) {
        gizmos.circle_2d(food.pos, food.radius, food.color);
    }
}
fn food_update(
    mut gizmos: Gizmos,
    bound_query: Query<&mut Bound>,
    mut food_query: Query<(&mut Food, &mut Transform)>,
    mut snake_query: Query<&mut SnakeModel>,
    mut score_query: Query<(&mut Text, &mut Score)>,
    query: Query<&VisualDiagnostic>

) {
    for (mut food, mut transform) in &mut food_query {
        for mut snake in &mut snake_query {
            if snake_eats_food(&snake, &food) {
                food.direction = new_food_direction();
                food.pos = new_food_position();
                food.color = new_food_color();

                for (mut text, mut score) in &mut score_query {
                    score.score_num += 1;
                    let score_string = score.score_num.to_string();
                    text.sections[0].value = format!("Score: {score_string}");
                }
                snake.size += 1.0;
                break;
            }
        }

        draw_food(&mut food, &mut gizmos, &query);

        food_on_bound(&mut food, &bound_query);

        transform.translation = Vec3::new(food.pos.x, food.pos.y, 0.0);
    }
}