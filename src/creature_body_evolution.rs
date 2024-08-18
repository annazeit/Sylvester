use bevy::prelude::Entity;

use crate::snake_model::{SnakeSpireNode, SnakeSpireNodeType};

type SpawnSnakeSpineNode = fn() -> Entity;

fn spine_from_size(size: i32, mut spawn: SpawnSnakeSpineNode) -> Vec<SnakeSpireNode> {
    if size > 20 { 
        let big_entity = spawn();
        return vec! [ 
            SnakeSpireNode {
                distance_from_head: 0.0,
                node_type: SnakeSpireNodeType::Big(big_entity)
            },
            SnakeSpireNode {
                distance_from_head: 50.0,
                node_type: SnakeSpireNodeType::Medium,
            },
            SnakeSpireNode {
                distance_from_head: 70.0,
                node_type: SnakeSpireNodeType::Small 
            }
        ]; 
    }
    // if size > 10 { return vec! [ SnakeSpireNodeType::Medium, SnakeSpireNodeType::Small, SnakeSpireNodeType::Small ]; }
    // if size > 10 { return vec! [ SnakeSpireNodeType::Small, SnakeSpireNodeType::Small, SnakeSpireNodeType::Small ]; }
    // if size > 5 { return vec! [ SnakeSpireNodeType::Medium, SnakeSpireNodeType::Small ]; }
    return vec! [ 
        SnakeSpireNode {
            distance_from_head: 0.0,
            node_type: SnakeSpireNodeType::Small 
        },
        SnakeSpireNode {
            distance_from_head: 10.0,
            node_type: SnakeSpireNodeType::Small 
        }
    ] ;
}

fn node_radius(node: SnakeSpireNodeType) -> f32 {
    match node {
        SnakeSpireNodeType::Small => { 10.0 }
        SnakeSpireNodeType::Medium => { 15.0 }
        SnakeSpireNodeType::Big(_) => { 20.0 }
    }
}