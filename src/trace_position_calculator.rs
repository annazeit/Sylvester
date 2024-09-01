use std::{collections::LinkedList, f32::consts::PI};
use bevy::math::{Vec2, VectorSpace};
use std::f32::*;

fn vec_angle (pos: Vec2) -> Option<f32>{
    if pos.x > 0.0 {
        if pos.y > 0.0 {
            return Some(f32::atan(pos.y / pos.x)); // right top
        }
        else if pos.y < 0.0 {
            return Some(f32::atan(pos.y / pos.x)); // right bottom
        }
        else { return Some(0.0)} // right middle
    }
    else if pos.x < 0.0 {
        if pos.y > 0.0 {
            return Some(PI + f32::atan(pos.y / pos.x)); // left top
        }
        else if pos.y < 0.0 {
            return Some(-PI + f32::atan(pos.y / pos.x)); // left bottom
        }
        else {return Some(PI);} // left middle
    }
    else { 
        if pos.y > 0.0 {
            return Some(PI / 2.0); // middle top
        }
        else if pos.y < 0.0 {
            return Some(-PI / 2.0); // middle bottom
        }
        else {return None;} // origin
    }

}

pub struct CalculatedDirections {
    // direction angle on result trace segment
    pub direction_current: f32,
    
    // direction angle on the previous trace segment. 
    // If prevous segment doesn't exist then returns current segment direction angle. 
    pub direction_previous: f32,
    
    // direction angle on the next trace segment. 
    // If next segment doesn't exist then returns current segment direction angle.
    pub direction_next: f32,

    // Current trace segment is a line. 
    // If result position is in the beginning of the secment then fraction is 0.
    // If result position is in the middle if the segment then fraction is 0.5.
    // If result positon is tn the end of the segment is 1.
    pub segment_distance_fraction: f32
}

pub struct CalculationResult {
    pub position: Vec2,
    pub directions: CalculatedDirections
}

/// Input angles must be in radians.
/// Result vector will be zero or positive number less than PI / 2.
pub fn angle_radian_normalize(a: f32) -> f32 {
    let result = {
        let a = a % (PI * 2.0);
        if a >= 0.0 { a }
        else { PI * 2.0 + a }
    };
    debug_assert!(result >= 0.0 && result < PI * 2.0);
    result
}

//calculates node_pos so it can be drawn in the trace
pub fn calculate_node_pos_traced_on_distance_from_head (
    head_pos: Vec2, 
    head_direction: f32,
    trace: impl Iterator<Item = Vec2>, 
    trace_length: usize,
    distance_from_head: f32
) -> CalculationResult {
    let mut direction_current = head_direction;
    let mut direction_previous = head_direction; // remember to interpolate current angle toward previous angle
    let mut checkpoint_previous: Vec2 = head_pos; // starting pos, which will then change 
    let mut total_distance = distance_from_head; // distance between head_pos and node

    // for each index and value in LinkedList 
    let mut iterator = trace.into_iter();
    loop { 
        match iterator.next() {
            None => { 
                return CalculationResult {
                    position: checkpoint_previous, 
                    directions: CalculatedDirections {
                        direction_current: angle_radian_normalize(direction_current),
                        direction_previous: angle_radian_normalize(direction_previous),
                        direction_next: angle_radian_normalize(direction_current),
                        segment_distance_fraction: 1.0
                    }
                };
            }
            Some(checkpoint) => {
                let delta_vec = checkpoint - checkpoint_previous; // step between current_pos and checkpoint
                let delta_len = delta_vec.length(); // step length

                match vec_angle(-delta_vec) {
                    Some(direction) => {
                        direction_previous  = direction_current;
                        direction_current = direction
                    },
                    None => {}
                }

                if total_distance <= delta_len { // going to exit if total_distance is less than step
                    let delta_vec_norm = delta_vec / delta_len; // normalise step
                    let last_delta_vec = delta_vec_norm * total_distance; // multiply step by whats left from total_distance
                    let position_result = checkpoint_previous + last_delta_vec; 

                    let direction_next = 
                        match iterator.next() {
                            None => { direction_current }
                            Some (position_next) => {
                                match vec_angle(position_result - position_next) {
                                    Some(direction) => { direction },
                                    None => { direction_current }
                                }
                            }
                        };

                    let segment_distance_fraction = total_distance / delta_len;

                    return CalculationResult{
                        position: position_result, 
                        directions: CalculatedDirections {
                            direction_current: angle_radian_normalize(direction_current),
                            direction_previous: angle_radian_normalize(direction_previous),
                            direction_next: angle_radian_normalize(direction_next),
                            segment_distance_fraction
                        }
                    };
                }
                else { // update current position by step
                    checkpoint_previous = checkpoint; 
                    total_distance -= delta_len; // decrease total_distance by step
                }; 
            }
        }
    }
    
}

#[cfg(test)]
mod tests {
    use crate::snake_model::TraceItem;

    use super::*;

    fn assert_vec2_eq(a: Vec2, b: Vec2) {
        assert_float_eq(a.x, b.x);
        assert_float_eq(a.y, b.y)
    }

    fn assert_float_eq(a: f32, b: f32) {
        let delta_max = 0.001;
        let c = f32::abs(a - b);
        assert!(c < delta_max)
    }

    // test how to create linked list from TraceItem, create iterator and map trace postions iterator.
    #[test]
    fn linked_list_map() {
        let list = LinkedList::from([ TraceItem {  index: 1, pos: Vec2::ZERO }]);
        let trace_positions = list.iter().map(|p| p.pos);
        let head_pos = Vec2::new(0.0, 0.0);

        // linked list of Trace Items was casted to iterator, extracted positions using map and iterated inside of calculator.
        let _result = calculate_node_pos_traced_on_distance_from_head(
            head_pos, 
            PI / 2.0,
            trace_positions, 
            list.len(), 
            10.0
        );

    }

    #[test]
    fn basic_test() {
        let trace: [Vec2; 1] = [
            Vec2::new(5.0, 0.0)
        ];
        let actual = calculate_node_pos_traced_on_distance_from_head(
            Vec2::new(0.0, 0.0), 
            PI,
            LinkedList::from(trace).into_iter(), 
            trace.len(), 
            3.0
        );
        let expected_result = Vec2::new(3.0, 0.0);
        assert_eq!(actual.position, expected_result);
        assert_eq!(actual.directions.direction_current, PI);
        assert_eq!(actual.directions.direction_previous, PI);
        assert_eq!(actual.directions.direction_next, PI)
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
            let actual = calculate_node_pos_traced_on_distance_from_head(
                head_pos, 
                PI / 2.0,
                LinkedList::from(trace).into_iter(), 
                trace.len(), 
                distance_from_head
            );
            assert_eq!(actual.position, expected_pos);
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
            let actual = calculate_node_pos_traced_on_distance_from_head(
                head_pos, 
                PI / 2.0,
                LinkedList::from(trace).into_iter(), 
                trace.len(), 
                distance_from_head
            );
            assert_vec2_eq(actual.position, expected_pos);

            assert_eq!(actual.directions.direction_current, 0.0);
            assert_eq!(actual.directions.direction_previous, PI / 2.0);
            assert_eq!(actual.directions.direction_current, 0.0);
        }
        
    }
    
    #[test]
    fn thirty_degree_test() {
        let triangle_hypotenuse = 20.0;
        let triangle_side_oposite = 10.0;
        let triangle_side_adjesent =  17.32051;
        let angle_30_degree_in_radians = 0.523599;

        let head_pos = Vec2::new(10.0, 0.0);
        let trace = [
            Vec2::new(0.0, 0.0),
            Vec2::new(triangle_side_adjesent, triangle_side_oposite),
            
        ];

        let expected_results = [
            (triangle_hypotenuse + head_pos.x, Vec2::new(triangle_side_adjesent, triangle_side_oposite)),
        ];

        for (distance_from_head, expected_pos) in expected_results {
            let actual = calculate_node_pos_traced_on_distance_from_head(
                head_pos, 
                PI / 2.0,
                LinkedList::from(trace).into_iter(), 
                trace.len(), 
                distance_from_head
            );
            assert_vec2_eq(actual.position, expected_pos);
            let direction_current_expected = -PI + angle_30_degree_in_radians;
            assert_float_eq(actual.directions.direction_current, direction_current_expected);
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
            let actual = calculate_node_pos_traced_on_distance_from_head(
                head_pos, 
                PI / 2.0,
                LinkedList::from(trace).into_iter(), 
                trace.len(), 
                distance_from_head
            );
            assert_vec2_eq(actual.position, expected_pos);
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
            let actual = calculate_node_pos_traced_on_distance_from_head(
                head_pos, 
                PI / 2.0,
                LinkedList::from(trace).into_iter(), 
                trace.len(), 
                distance_from_head
            );
            assert_vec2_eq(actual.position, expected_pos);
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
            let actual = calculate_node_pos_traced_on_distance_from_head(
                head_pos, 
                PI / 2.0,
                LinkedList::from(trace).into_iter(), 
                trace.len(), 
                distance_from_head
            );

            println!("!!!actual_pos: {:?}", actual.position);
            println!("!!!expected_pos: {:?}", expected_pos);
            assert_vec2_eq(actual.position, expected_pos);
        }
    }

    #[test]
    fn positive_and_zero() {
        let pos = Vec2::new(10.0, 0.0);
        let actual = vec_angle(pos);
        let atan = f32::atan2(pos.y, pos.x); // 0.0
        let expected = Some(atan); 
        println!("{:?}", expected);
        assert_eq!(actual, expected);
    }    
    #[test]
    fn negative_and_zero() {
        let pos = Vec2::new(-10.0, 0.0);
        let actual = vec_angle(pos);
        let atan = f32::atan2(pos.y, pos.x); // PI
        let expected = Some(atan);
        println!("{:?}", expected);
        assert_eq!(actual, expected);
    }    
    #[test]
    fn zero_and_positive() {
        let pos = Vec2::new(0.0, 10.0);
        let actual = vec_angle(pos);
        let atan = f32::atan2(pos.y, pos.x); // PI / 2.0
        let expected = Some(atan); 
        println!("{:?}", expected); 
        assert_eq!(actual, expected);
    }   
    #[test]
    fn zero_and_negative() {
        let pos = Vec2::new(0.0, -10.0);
        let actual = vec_angle(pos);
        let atan = f32::atan2(pos.y, pos.x); // -PI / 2.0
        let expected = Some(atan);
        println!("{:?}", expected); 
        assert_eq!(actual, expected);
    }    
    #[test]
    fn positive_and_positive() {
        let pos = Vec2::new(10.0, 10.0);
        let actual = vec_angle(pos);
        let atan = f32::atan2(pos.y, pos.x); // PI / 4.0
        let expected = Some(atan);
        println!("{:?}", expected);
        assert_eq!(actual, expected);
    }    
    #[test]
    fn positive_and_negative() {
        let pos = Vec2::new(10.0, -10.0);
        let actual = vec_angle(pos);
        let atan = f32::atan2(pos.y, pos.x); // -PI / 4.0
        let expected = Some(atan);
        println!("{:?}", expected);
        assert_eq!(actual, expected);
    }
    #[test]
    fn negative_and_negative() {
        let pos = Vec2::new(-10.0, -10.0);
        let actual = vec_angle(pos);
        let atan = f32::atan2(pos.y, pos.x); // -PI + PI / 4.0
        let expected = Some(atan);
        println!("{:?}", expected);
        assert_eq!(actual, expected);
    }
    #[test]
    fn negative_and_positive() {
        let pos = Vec2::new(-10.0, 10.0);
        let actual = vec_angle(pos);
        let atan = f32::atan2(pos.y, pos.x); // PI - PI / 4.0
        let expected = Some(atan);
        println!("{:?}", expected);
        assert_eq!(actual, expected);
    }
    #[test]
    fn zero_and_zero() {
        let pos = Vec2::new(0.0,0.0);
        let actual = vec_angle(pos);
        let expected = None;
        assert_eq!(actual, expected);
    }
}