use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::palettes::basic::RED;
use bevy::color::{Color, Srgba};
use bevy::math::Vec2;
use bevy::prelude::*;
use rand::Rng;
use crate::snake_model::SnakeHead;

pub struct FoodPlugin;

#[derive(Component)]
pub struct Food {
    food_pos: Vec2,
    direction: f32,
    radius: f32,
    color: Srgba,
}
#[derive(Component)]
pub struct Bound {
    bound_pos: Vec2,
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
        app.add_systems(Update, draw_food);
        app.add_systems(Update, draw_bound);
    }
}

fn bound_start(mut commands: Commands) {
    commands.spawn(Bound{
        bound_pos: Vec2::new(0.0, 0.0),
        radius: 500.0,
    });
}
fn score_start(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("C:/Users/annaz/MovistarTextRegular.ttf");
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
fn food_start (mut commands: Commands) {
    let mut rnd = rand::thread_rng();
    for _ in 0..5 {
        let hue: f32 = rnd.gen();
        let color: Srgba = Color::hsl(hue * 360.0, 0.95, 0.7).to_srgba();
        commands.spawn(Food {
            food_pos: new_food_position(),
            direction: new_food_direction(),
            radius: 10.0,
            color,
        });
    }
}

fn draw_bound(
    mut gizmos: Gizmos,
    bound_query: Query<&mut Bound>,
) {
    for bound in &bound_query{
        gizmos.circle_2d(bound.bound_pos, bound.radius, RED);
    }
}

fn new_food_position() -> Vec2 {
    let x = rand::thread_rng().gen_range(-300..=300) as f32;
    let y = rand::thread_rng().gen_range(-300..=300) as f32;
    Vec2::new(x, y)
}
fn new_food_direction() -> f32 {
    let num = rand::thread_rng().gen_range(-180..=180) as f32;
    num
}

fn food_is_eaten_by_any_snake(food: &Food, snake_query: &mut Query<&mut SnakeHead>) -> bool {
    for snake in snake_query {
        if snake_eats_food(&snake, food) {
            return true;
        }
    }
    return false;
}

fn snake_eats_food(
    snake: &SnakeHead,
    food: &Food
) -> bool {
    let distance_vector = snake.head_pos - food.food_pos;
    let distance_between = ((distance_vector.x * distance_vector.x) + (distance_vector.y * distance_vector.y)).sqrt();
    distance_between < food.radius + snake.head_radius
}

fn draw_food(
    mut gizmos: Gizmos,
    bound_query: Query<&mut Bound>,
    mut food_query: Query<&mut Food>,
    mut snake_query: Query<&mut SnakeHead>,
    mut score_query: Query<(&mut Text, &mut Score)>,
) {
    for mut food in &mut food_query {
        if food_is_eaten_by_any_snake(&food, &mut snake_query) {
            food.food_pos = new_food_position();
            food.direction = new_food_direction();

            for (mut text, mut score) in &mut score_query {
                score.score_num += 1;
                let score_string = score.score_num.to_string();
                text.sections[0].value = format!("Score: {score_string}");
            }
        }

        let food_move = {
            let x = f32::sin(food.direction);
            let y = f32::cos(food.direction);
            Vec2::new(x, y)
        };

        food.food_pos += food_move;
        gizmos.circle_2d(food.food_pos, food.radius, food.color);

        for bound in &bound_query {
            let origin = Vec2::new(0.0, 0.0);
            let distance_from_origin_to_food: f32 = {
                let distance_vector = origin - food.food_pos;
                ((distance_vector.x * distance_vector.x) + (distance_vector.y * distance_vector.y)).sqrt()
            };
            if distance_from_origin_to_food > (bound.radius - (food.radius * 2.0 )) {
                food.direction = new_food_direction()
            }
        }
    }
}