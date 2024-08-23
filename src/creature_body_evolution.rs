use bevy::prelude::*;
use bevy::{sprite::SpriteBundle};
use crate::snake_model::{SnakeModel, SnakeSpineNode, SnakeSpineNodeType as SnakeSpineNodeType};

type SpawnSnakeSpineNode = fn() -> Entity;


/// All creature visual movable parts will have this component to query their transformations.  
#[derive(Component)]
pub struct CreatureBodyVisualElement;

pub fn spine_from_size(mut commands: &mut Commands,  asset_server: &Res<AssetServer>, mut snake: &mut SnakeModel) -> Vec<SnakeSpineNode> {
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
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.1,0.1, 0.1)),
            ..default()
        },
        CreatureBodyVisualElement
    )).id();

    let mut list: Vec<SnakeSpineNode> = Vec::new();
    list.push(SnakeSpineNode {
        distance_from_head: 0.0,
        node_type: head_entity,
    });

    for i in 0..100 {
    let node_entity = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("SpinePart.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.2, 0.2, 0.0)),
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

fn node_radius(node: SnakeSpineNodeType) -> f32 {
    match node {
        SnakeSpineNodeType::Small => { 10.0 }
        SnakeSpineNodeType::Medium => { 15.0 }
        SnakeSpineNodeType::Big => { 20.0 }
    }
}