use std::collections::LinkedList;
use bevy::math::Vec2;

// calculates node_pos so it can be drawn in the trace
pub fn calculate_node_pos_traced_on_distance_from_head (
    head_pos: Vec2, 
    trace: &mut impl Iterator<Item = Vec2>, 
    trace_length: usize,
    distance_from_head: f32
) -> Vec2 {
    let mut current_pos: Vec2 = head_pos; // starting pos, which will then change 
    let mut total_distance = distance_from_head; // distance between head_pos and node
    let last_trace_index = trace_length - 1;

    // for each index and value in LinkedList 
    for (checkpoint_index, checkpoint) in trace.enumerate() { 
        let delta_vec = checkpoint - current_pos; // step between current_pos and checkpoint
        let delta_len = delta_vec.length(); // step length

        if total_distance < delta_len || checkpoint_index == last_trace_index{ // if total_distance is bigger than step OR checkpoint_index is last one in linkedList
            let delta_vec_norm = delta_vec / delta_len; // normalise step
            let last_delta_vec = delta_vec_norm * total_distance; // multiply step by whats left from total_distance
            current_pos += last_delta_vec; 
            break;
        }
        else {
            current_pos = checkpoint; 
            total_distance -= delta_len; // decrease total_distance by step
        } 
    }
    return current_pos;
}

#[cfg(test)]
mod tests {
    use bevy::math::VectorSpace;

    use crate::snake_model::TraceItem;

    use super::*;

    fn assert_vec2_eq(a: Vec2, b: Vec2){
        let delta_max: f32 = 0.001;
        let dx = f32::abs(a.x - b.x);
        assert!(dx < delta_max);
        let dy = f32::abs(a.y - b.y);
        assert!(dy < delta_max);
    }

    // test how to create linked list from TraceItem, create iterator and map trace postions iterator.
    #[test]
    fn linked_list_map() {
        let list = LinkedList::from([ TraceItem {  index: 1, pos: Vec2::ZERO }]);
        let mut trace_positions = list.iter().map(|p| p.pos);
        let head_pos = Vec2::new(0.0, 0.0);

        // linked list of Trace Items was casted to iterator, extracted positions using map and iterated inside of calculator.
        let _result = calculate_node_pos_traced_on_distance_from_head(head_pos, &mut trace_positions, list.len(), 10.0);
    }

    #[test]
    fn basic_test() {
        let trace: [Vec2; 1] = [
            Vec2::new(5.0, 0.0)
        ];
        let actual_result = calculate_node_pos_traced_on_distance_from_head(
            Vec2::new(0.0, 0.0), 
            &mut LinkedList::from(trace).into_iter(), 
            trace.len(), 
            3.0
        );
        let expected_result = Vec2::new(3.0, 0.0);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn segment_smaller_than_distance_from_head_test() {
        let head_pos = Vec2::new(0.0, 0.0);
        let trace = [
            Vec2::new(5.0, 5.0),
            
        ];

        let expected_results = [
            (21.21320344, Vec2::new(15.0, 15.0)),
        ];

        for (distance_from_head, expected_pos) in expected_results {
            let actual_pos = calculate_node_pos_traced_on_distance_from_head(
                head_pos, 
                &mut LinkedList::from(trace).into_iter(), 
                trace.len(), 
                distance_from_head
            );
            assert_eq!(actual_pos, expected_pos);
        }
        
    }
    
    #[test]
    fn up_down_left_right_test() {
        let head_pos = Vec2::new(0.0, 0.0);
        let trace = [
            // y + 10
            Vec2::new(0.0, 10.0),
            // x + 20
            Vec2::new(20.0, 10.0),
            // y - 30
            Vec2::new(20.0, -20.0),
            // x - 15
            Vec2::new(5.0, -20.0),
            
        ];

        let expected_results = [
            (75.0, Vec2::new(5.0, -20.0)),
        ];

        for (distance_from_head, expected_pos) in expected_results {
            let actual_pos = calculate_node_pos_traced_on_distance_from_head(
                head_pos, 
                &mut LinkedList::from(trace).into_iter(), 
                trace.len(), 
                distance_from_head
            );
            assert_eq!(actual_pos, expected_pos);
        }
        
    }
    
    #[test]
    fn thirty_degree_test() {
        let head_pos = Vec2::new(10.0, 0.0);
        let trace = [
            Vec2::new(0.0, 0.0),
            Vec2::new(17.32051, 10.0),
            
        ];

        let expected_results = [
            (30.0, Vec2::new(17.32051, 10.0)),
        ];

        for (distance_from_head, expected_pos) in expected_results {
            let actual_pos = calculate_node_pos_traced_on_distance_from_head(
                head_pos, 
                &mut LinkedList::from(trace).into_iter(), 
                trace.len(), 
                distance_from_head
            );
            assert_vec2_eq(actual_pos, expected_pos);
        }
        
    }

    #[test]
    fn one_hudred_ten_degree_test() {
        let head_pos = Vec2::new(10.0, 0.0);
        let trace = [
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, -17.32051),
            
        ];

        let expected_results = [
            (30.0, Vec2::new(10.0, -17.32051)),
        ];

        for (distance_from_head, expected_pos) in expected_results {
            let actual_pos = calculate_node_pos_traced_on_distance_from_head(
                head_pos, 
                &mut LinkedList::from(trace).into_iter(), 
                trace.len(), 
                distance_from_head
            );
            assert_vec2_eq(actual_pos, expected_pos);
        }
        
    }
    #[test]
    fn square_test() {
        let head_pos = Vec2::new(0.0, 0.0);
        let trace = [
            Vec2::new(0.0, 10.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(10.0, -10.0),
            Vec2::new(0.0, -10.0),
            Vec2::new(-10.0, -10.0),
            Vec2::new(-10.0, 0.0),
            Vec2::new(-10.0, 10.0),
            Vec2::new(0.0, 10.0),

            
        ];

        let expected_results = [
            (10.0, Vec2::new(0.0, 10.0)),
        ];

        for (distance_from_head, expected_pos) in expected_results {
            let actual_pos = calculate_node_pos_traced_on_distance_from_head(
                head_pos, 
                &mut LinkedList::from(trace).into_iter(), 
                trace.len(), 
                distance_from_head
            );
            assert_vec2_eq(actual_pos, expected_pos);
        }
        
    }

    #[test]
    fn debug_strage_case() {
        let head_pos = Vec2::new(0.0, 107.59321);
        let trace = [
            Vec2::new(0.0, 102.58502),
            Vec2::new(0.0, 92.575264),
            Vec2::new(0.0, 82.560165),
            Vec2::new(0.0, 72.557556),
            Vec2::new(0.0, 62.554504),
            Vec2::new(0.0, 52.541656),
            Vec2::new(0.0, 42.534496),
            Vec2::new(0.0, 32.525833),
            Vec2::new(0.0, 22.51635),
            Vec2::new(0.0, 10.018846),
            Vec2::new(0.0, -10.0),
            
        ];

        let expected_results = [
            (107.59321, Vec2::new(0.0, -0.0)),
        ];

        for (distance_from_head, expected_pos) in expected_results {
            let actual_pos = calculate_node_pos_traced_on_distance_from_head(
                head_pos, 
                &mut LinkedList::from(trace).into_iter(), 
                trace.len(), 
                distance_from_head
            );

            println!("!!!actual_pos: {:?}", actual_pos);
            println!("!!!expected_pos: {:?}", expected_pos);
            assert_vec2_eq(actual_pos, expected_pos);
        }
    }    
    
}