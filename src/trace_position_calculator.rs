use std::collections::LinkedList;
use bevy::math::Vec2;

// calculates node_pos so it can be drawn in the trace
pub fn calculate_node_pos_traced_on_distance_from_head (head_pos: Vec2, trace: LinkedList<Vec2>, distance_from_head: f32) -> Vec2 {
    let mut current_pos: Vec2 = head_pos; // starting pos which will then change 
    let mut total_distance = distance_from_head; // distance between head_pos and our node
    let last_trace_index = trace.len() - 1;

    // for each index and value in LinkedList 
    for (checkpoint_index, checkpoint) in trace.iter().enumerate() { 
        let delta_vec = *checkpoint - current_pos; // step between our current_pos and checkpoint
        let delta_len = delta_vec.length();

        if total_distance < delta_len || checkpoint_index == last_trace_index{ // if total_distance is bigget than step OR checkpoint_index is last one in linkedList
            let delta_vec_norm = delta_vec / delta_len; // normalise step
            let last_delta_vec = delta_vec_norm * total_distance; // multiply step by whats left from total_distance
            current_pos += last_delta_vec;  
            break;
        }
        else {
            current_pos = *checkpoint; 
            total_distance -= delta_len; // decrease total_distance by step
        } 
    }
    return current_pos;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_vec2_eq(a: Vec2, b: Vec2){
        let delta_max: f32 = 0.001;
        let dx = f32::abs(a.x - b.x);
        assert!(dx < delta_max);
        let dy = f32::abs(a.y - b.y);
        assert!(dy < delta_max);
    }

    #[test]
    fn basic_test() {
        let trace = [
            Vec2::new(5.0, 0.0)
        ];
        let actual_result = calculate_node_pos_traced_on_distance_from_head(Vec2::new(0.0, 0.0), LinkedList::from(trace), 3.0);
        let expected_result = Vec2::new(3.0, 0.0);
        assert_eq!(actual_result, expected_result);
    }
}