use bevy::app::{App, Plugin, Startup};
use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, score_start);
    }
}

// pub(super) rather than private: food_item.rs (a sibling module) needs to name this
// type for its Query<(&mut Text, &mut Score)> parameter, even though it never touches
// the field directly - only add_point below does that.
#[derive(Component)]
pub(super) struct Score {
    score_num: i32
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

// Called from food.rs when normal (non-poisonous) food is eaten.
pub(super) fn add_point(score_query: &mut Query<(&mut Text, &mut Score)>) {
    for (mut text, mut score) in score_query {
        score.score_num += 1;
        let score_string = score.score_num.to_string();
        text.sections[0].value = format!("Score: {score_string}");
    }
}
