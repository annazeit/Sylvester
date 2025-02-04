use bevy::prelude::Entity;
use bevy::prelude::Component;

/// Global game component.
#[derive(Component)]
pub struct TheGame {
    // NodeBundle is permanent UI root component.
    pub root_ui_node: Entity,

    /// If Some then point to Start button entity id. 
    /// If None then game is running.
    pub start_button_entity: Option<Entity>,
}