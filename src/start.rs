use bevy::{color::palettes::basic::*, gizmos, math::VectorSpace, prelude::*};

pub struct StartPlugin;

#[derive(Component)]
pub struct StartVisualDiagnostic {
    enabled: bool,
}

impl Plugin for StartPlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, button_start);
        app.add_systems(Update, button_system);
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button_start(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(StartVisualDiagnostic{
        enabled: false,
    });
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load("MovistarTextRegular.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

/// Visual diagnostics enabled when we need to draw extra entity shapes using gizmos to help to debug game.
/// Without visual diagnostics gizmos will not be used and we will see only SpriteBundles.
/// Drawing gizmos with SpriteBundles together helps to test the game.
pub fn start_button_draw_visual_diagnostics_info(query: &Query<&StartVisualDiagnostic>) -> bool {
    for button in query {
        if button.enabled == true { 
            return true;
        }
    }
    return false;
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut start_button_query: Query<&mut StartVisualDiagnostic>,
    mut gizmos: Gizmos
) {
    for mut button in &mut start_button_query {
        for (interaction, mut color, mut border_color, children) in &mut interaction_query {
            match *interaction {
                Interaction::Pressed => {
                    *color = PRESSED_BUTTON.into();
                    border_color.0 = RED.into();
                    button.enabled = true;   
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                    border_color.0 = Color::BLACK;
                }
            }
        }
    }
}
