use bevy::prelude::*;
use bevy::{sprite::SpriteBundle};
use crate::snake_model::{SnakeModel, SnakeSpineNode, SnakeSpineNodeType as SnakeSpineNodeType};
#[cfg(test)]
use crate::snake_model::snake_model_new;

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
// SpineEnd.png's ring renders larger than SpinePart.png's at the same scale, so it
// gets its own smaller value here - tune by eye against SpinePart's ring size.
pub const BASE_END_SPRITE_SCALE: f32 = 0.12;

// Off-screen holding spot for body segments not currently part of the visible
// creature - used at spawn time here, and in snake_extension.rs draw_nodes to
// park segments that fall out of range when the creature shrinks (e.g. poison food).
pub const PARKED_SEGMENT_POSITION: Vec3 = Vec3::new(1000.0, 0.0, 0.0);

// Maps the snake's current size to its evolution tier.
pub fn tier_for_size(size: f32) -> SnakeSpineNodeType {
    if size >= BIG_TIER_MIN_SIZE { SnakeSpineNodeType::Big }
    else if size >= MEDIUM_TIER_MIN_SIZE { SnakeSpineNodeType::Medium }
    else { SnakeSpineNodeType::Small }
}

// Seconds it takes node_radius to animate into a newly reached tier.
pub const SCALE_TRANSITION_DURATION: f32 = 0.4;

// Eases 0..1 progress so growth accelerates then decelerates instead of moving at a constant rate.
pub fn ease_smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

// Advances the creature's size-tier evolution once per frame: detects a tier change
// (kicking off a fresh transition from the current radius) and eases node_radius
// toward the current tier's target radius over SCALE_TRANSITION_DURATION.
pub fn update_evolution_transition(snake: &mut SnakeModel, delta_seconds: f32) {
    let target_tier = tier_for_size(snake.size);
    if target_tier != snake.evolution_tier {
        snake.evolution_transition_start_radius = snake.node_radius;
        snake.evolution_tier = target_tier;
        snake.evolution_transition_elapsed = 0.0;
    }
    let target_radius = node_radius(snake.evolution_tier);
    if snake.evolution_transition_elapsed < SCALE_TRANSITION_DURATION {
        let t = ease_smoothstep(snake.evolution_transition_elapsed / SCALE_TRANSITION_DURATION);
        snake.node_radius = snake.evolution_transition_start_radius + (target_radius - snake.evolution_transition_start_radius) * t;
        snake.evolution_transition_elapsed += delta_seconds;
    } else {
        snake.node_radius = target_radius;
    }
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
            transform: Transform::from_translation(PARKED_SEGMENT_POSITION).with_scale(Vec3::splat(BASE_HEAD_SPRITE_SCALE)),
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
        list.push(spawn_body_node(commands, asset_server));
    }

    return list;
}

fn spawn_body_node(commands: &mut Commands, asset_server: &Res<AssetServer>) -> SnakeSpineNode {
    let node_entity = commands.spawn((
        SpriteBundle {
            texture: asset_server.load("SpinePart.png"),
            transform: Transform::from_translation(PARKED_SEGMENT_POSITION).with_scale(Vec3::new(BASE_BODY_SPRITE_SCALE, BASE_BODY_SPRITE_SCALE, 0.0)),
            ..default()
        },
        CreatureBodyVisualElement
    )).id();

    SnakeSpineNode {
        distance_from_head: 50.0,
        node_type: node_entity,
    }
}

// draw_nodes (snake_extension.rs) indexes snake.body[0..=snake.size], but the pool is
// only pre-spawned up to 100 segments. Tops it up on demand so eating enough food to
// exceed that no longer panics with an out-of-bounds index.
pub fn ensure_body_capacity(commands: &mut Commands, asset_server: &Res<AssetServer>, snake: &mut SnakeModel) {
    let needed = snake.size as usize + 1; // +1 for the head at index 0
    while snake.body.len() < needed {
        snake.body.push(spawn_body_node(commands, asset_server));
    }
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

    #[test]
    fn ease_smoothstep_endpoints_and_midpoint() {
        assert_eq!(ease_smoothstep(0.0), 0.0);
        assert_eq!(ease_smoothstep(1.0), 1.0);
        assert_eq!(ease_smoothstep(0.5), 0.5);
    }

    #[test]
    fn ease_smoothstep_clamps_out_of_range_input() {
        assert_eq!(ease_smoothstep(-1.0), 0.0);
        assert_eq!(ease_smoothstep(2.0), 1.0);
    }

    #[test]
    fn no_tier_change_settles_immediately_at_current_target() {
        let mut snake = snake_model_new(0); // starts at size 5.0, Small tier, already settled
        update_evolution_transition(&mut snake, 0.1);
        assert_eq!(snake.node_radius, BASE_NODE_RADIUS);
    }

    #[test]
    fn tier_change_eases_toward_target_instead_of_jumping() {
        let mut snake = snake_model_new(0);
        snake.size = MEDIUM_TIER_MIN_SIZE; // crosses into Medium tier
        update_evolution_transition(&mut snake, 0.1); // starts the transition (t=0 this frame)
        update_evolution_transition(&mut snake, 0.1); // elapsed has now advanced, t>0
        assert_eq!(snake.evolution_tier, SnakeSpineNodeType::Medium);
        assert!(snake.node_radius > BASE_NODE_RADIUS);
        assert!(snake.node_radius < node_radius(SnakeSpineNodeType::Medium));
    }

    #[test]
    fn transition_settles_at_target_once_duration_elapses() {
        let mut snake = snake_model_new(0);
        snake.size = MEDIUM_TIER_MIN_SIZE;
        update_evolution_transition(&mut snake, SCALE_TRANSITION_DURATION); // starts the transition; elapsed reaches the duration
        update_evolution_transition(&mut snake, 0.0); // elapsed is no longer < duration -> snaps to target
        assert_eq!(snake.node_radius, node_radius(SnakeSpineNodeType::Medium));
    }
}