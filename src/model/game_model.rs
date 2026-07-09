use bevy::prelude::Entity;
use bevy::prelude::Component;
use bevy::prelude::States;

/// Global game component.
#[derive(Component)]
pub struct TheGame {
    // NodeBundle is permanent UI root component.
    pub root_ui_node: Entity,

    /// If Some then point to Start button entity id.
    /// If None then game is running.
    pub start_button_entity: Option<Entity>,
}

/// Drives whether gameplay systems (snake/food) are allowed to run.
/// Starts on the menu; the Start button moves this to Playing.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Playing,
}