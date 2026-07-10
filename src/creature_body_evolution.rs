use bevy::prelude::*;
use bevy::{sprite::SpriteBundle};
use crate::snake_model::{SnakeModel, SnakeSpineNode, SnakeSpineNodeType as SnakeSpineNodeType};

/// All creature visual movable parts will have this component to query their transformations.
#[derive(Component)]
pub struct CreatureBodyVisualElement;

// Size (snake.size) at which the creature moves up to the next evolution tier.
const MEDIUM_TIER_MIN_SIZE: f32 = 10.0;
const BIG_TIER_MIN_SIZE: f32 = 25.0;

// Base values matching today's Small tier, used to scale sprites/spacing proportionally
// to node_radius as the creature evolves (see snake_extension.rs draw_nodes).
pub const BASE_NODE_RADIUS: f32 = 10.0;
pub const BASE_BODY_SPRITE_SCALE: f32 = 0.2;
pub const BASE_HEAD_SPRITE_SCALE: f32 = 0.1;

// Maps the snake's current size to its evolution tier.
pub fn tier_for_size(size: f32) -> SnakeSpineNodeType {
    if size >= BIG_TIER_MIN_SIZE { SnakeSpineNodeType::Big }
    else if size >= MEDIUM_TIER_MIN_SIZE { SnakeSpineNodeType::Medium }
    else { SnakeSpineNodeType::Small }
}

// Pre-spawns one head sprite plus 100 body-segment sprites up front (rather than
// spawning/despawning as the snake grows), returning them as SnakeSpineNodes.
// snake_extension.rs repositions these each frame based on snake.size and the trace.
pub fn spine_from_size(commands: &mut Commands,  asset_server: &Res<AssetServer>, snake: &mut SnakeModel) -> Vec<SnakeSpineNode> {
    // if size > 20 {
    //     let big_entity = spawn();
    //     return vec! [ 
    //         SnakeSpineNode {
    //             distance_from_head: 0.0,
    //             node_type: SnakeSpineNodeType::Big(big_entity)
    //         },
    //         SnakeSpineNode {
    //             distance_from_head: 50.0,
    //             node_type: SnakeSpineNodeType::Medium,
    //         },
    //         SnakeSpineNode {
    //             distance_from_head: 70.0,
    //             node_type: SnakeSpineNodeType::Small 
    //         }
    //     ]; 
    // }
    
    // let head_entity = commands.spawn((
    //         SpriteBundle {
    //             texture: asset_server.load("Test.png"),
    //             transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 1.0, 1.0)),
    //             ..default()
    //         },
    //         CreatureBodyVisualElement
    //     )).id();

    let head_entity = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("SpineHead.png"),
            transform: Transform::from_xyz(1000.0, 0.0, 0.0).with_scale(Vec3::splat(BASE_HEAD_SPRITE_SCALE)),
            ..default()
        },
        CreatureBodyVisualElement
    )).id();

    let mut list: Vec<SnakeSpineNode> = Vec::new();
    list.push(SnakeSpineNode {
        distance_from_head: 0.0,
        node_type: head_entity,
    });

    for _ in 0..100 {
    let node_entity = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("SpinePart.png"),
            transform: Transform::from_xyz(1000.0, 0.0, 0.0).with_scale(Vec3::new(BASE_BODY_SPRITE_SCALE, BASE_BODY_SPRITE_SCALE, 0.0)),
            ..default()
        },
        CreatureBodyVisualElement
    )).id();

    list.push(SnakeSpineNode {
        distance_from_head: 50.0,
        node_type: node_entity,
    });
    }
    
    return list;
}

pub fn node_radius(node: SnakeSpineNodeType) -> f32 {
    match node {
        SnakeSpineNodeType::Small => { BASE_NODE_RADIUS }
        SnakeSpineNodeType::Medium => { 15.0 }
        SnakeSpineNodeType::Big => { 20.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_tier_below_medium_threshold() {
        assert_eq!(tier_for_size(0.0), SnakeSpineNodeType::Small);
        assert_eq!(tier_for_size(9.9), SnakeSpineNodeType::Small);
    }

    #[test]
    fn medium_tier_at_and_above_threshold() {
        assert_eq!(tier_for_size(10.0), SnakeSpineNodeType::Medium);
        assert_eq!(tier_for_size(24.9), SnakeSpineNodeType::Medium);
    }

    #[test]
    fn big_tier_at_and_above_threshold() {
        assert_eq!(tier_for_size(25.0), SnakeSpineNodeType::Big);
        assert_eq!(tier_for_size(100.0), SnakeSpineNodeType::Big);
    }
}