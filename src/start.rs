use bevy::{color::palettes::basic::*, ecs::system::EntityCommands, gizmos, math::VectorSpace, prelude::*};

pub struct StartPlugin;

/// Global game component.
#[derive(Component)]
pub struct TheGame {
    // NodeBundle is permanent UI root component.
    pub root_ui_node: Entity,

    /// If Some then point to Start button entity id. 
    /// If None then geme is running.
    pub start_button_entity: Option<Entity>,
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

    // All UI must be under this root node component.
    let mut node_bundle_entity_commands: EntityCommands<'_> = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    });

    let mut the_game = TheGame { 
        root_ui_node: node_bundle_entity_commands.id(),
        start_button_entity: None 
    };
    create_start_button(&mut the_game, &mut node_bundle_entity_commands, &asset_server);
    commands.spawn(the_game);
}

fn create_start_button(the_game: &mut TheGame, node_bundle_entity_command: &mut EntityCommands<'_>, asset_server: &Res<AssetServer>) {
    node_bundle_entity_command.with_children(|parent| {
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
        the_game.start_button_entity = Some(start_button_bundle_entity_id);
        
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

fn despawn_start_button(
    the_game_query: &mut Query<&mut TheGame>,
    commands: &mut Commands,
){
    let mut the_game = the_game_query.single_mut();
    match the_game.start_button_entity {
        None => { 
            println!("No Start Game Button") 
        }
        Some (start_button_entity_value) => {
            // Remove first from parent and then remove button hierarchically
            // according to documentation https://bevy-cheatbook.github.io/fundamentals/hierarchy.html
            commands.entity(the_game.root_ui_node).remove_children(&[ start_button_entity_value ]);
            // start button has childer elements and must be despowned recursively.
            commands.entity(start_button_entity_value).despawn_recursive();
            the_game.start_button_entity = None;
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
