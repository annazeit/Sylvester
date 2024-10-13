use bevy::{color::palettes::basic::*, ecs::system::EntityCommands, gizmos, math::VectorSpace, prelude::*};

pub struct StartPlugin;

/// Global game component.
#[derive(Component)]
pub struct TheGame {
    // NodeBundle is permanent UI root component.
    pub root_ui_node: Entity,

    /// If Some then point to Start button entity id. 
    /// If None then geme is running.
    pub waitning_for_start: Option<Entity>,
}

#[derive(Component)]
pub struct StartVisualDiagnostic {
    enabled: bool,
}

impl Plugin for StartPlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, create_game);
        app.add_systems(Update, button_system);
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn create_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(StartVisualDiagnostic{
        enabled: false,
    });

    // All UI must be under this root node component.
    let mut root_ui_entity_commands: EntityCommands<'_> = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    });
    println!("node_bundle_entity: {}", root_ui_entity_commands.id().index().to_string());

    let mut the_game = TheGame { 
        root_ui_node: root_ui_entity_commands.id(),
        waitning_for_start: None 
    };
    create_start_button(&mut the_game, &mut root_ui_entity_commands, &asset_server);
    commands.spawn(the_game);
}

fn create_start_button(the_game: &mut TheGame, root_ui_entity_commands: &mut EntityCommands<'_>, asset_server: &Res<AssetServer>) {
    root_ui_entity_commands.with_children(|parent| {
        let mut start_button_bundle_entity = parent.spawn(ButtonBundle {
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
        });
        let start_button_bundle_entity_id = start_button_bundle_entity.id();
        println!("button_bundle_entity: {}", start_button_bundle_entity_id.index().to_string());
        the_game.waitning_for_start = Some(start_button_bundle_entity_id);
        
        start_button_bundle_entity.with_children(|parent| {
            let start_text_bundle_entity = parent.spawn(TextBundle::from_section(
                "Start",
                TextStyle {
                    font: asset_server.load("MovistarTextRegular.ttf"),
                    font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
            println!("start_text_bundle_entity: {}", start_text_bundle_entity.id().index().to_string());
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

fn despawn_start_button(
    the_game_query: &mut Query<&mut TheGame>,
    commands: &mut Commands,
){
    let mut the_game = the_game_query.single_mut();
    match the_game.waitning_for_start {
        None => { 
            println!("No Start Game Button") 
        }
        Some (waiting_for_start) => {
            // Remove first from parent and then remove button hierarchically
            // according to documentation https://bevy-cheatbook.github.io/fundamentals/hierarchy.html
            commands.entity(the_game.root_ui_node).remove_children(&[ waiting_for_start ]);
            // start button has childer elements and must be despowned recursively.
            commands.entity(waiting_for_start).despawn_recursive();
            the_game.waitning_for_start = None;
        }
    }
}

pub fn button_system(
    mut the_game_query: Query<&mut TheGame>,
    mut start_geme_button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
) {
    for (interaction, mut color, mut border_color) in &mut start_geme_button_query {
        match *interaction {
            Interaction::Pressed => {
                despawn_start_button(&mut the_game_query, &mut commands);
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
