
use std::f32::consts::PI;
use crate::trace_position_calculator::angle_average_calculator;

const interpolate_direction_precitions: f32 = 0.05;

pub fn interpolate_direction(prev_angle: f32, curr_angle: f32, next_angle: f32, target: f32) -> f32 { // returns target angle
    let mut a: f32;
    let mut b: f32;
    
    let mut a_angle: f32;
    let mut b_angle: f32;

    if target < 0.5 {
        a = 0.0;
        b = 0.5;
        a_angle = angle_average_calculator(&vec![prev_angle, curr_angle]);
        b_angle = curr_angle;
    }
    else if target > 0.5 {
        a = 0.5;
        b = 1.0;
        a_angle = curr_angle;
        b_angle = angle_average_calculator(&vec![curr_angle, next_angle]);
    }
    else {
        return  curr_angle;
    }

    loop {
        if (target - a).abs() < interpolate_direction_precitions {
            return a_angle;
        }
        else if (b - target).abs() < interpolate_direction_precitions {
            return b_angle;
        }

        let m = a + (b - a) / 2.0;
        let m_angle = angle_average_calculator(&vec![a_angle, b_angle]);

        if m > target {
            b = m;
            b_angle = m_angle;
        }
        else if m < target {
            a = m;
            a_angle = m_angle;
        }
        else {
            return  m_angle;
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn assert_float_equal(a: f32, b: f32) -> bool{
        let delta_max = 0.1;
        let c = f32::abs(a - b);
        assert!(c < delta_max);
        return true;
    }

    #[test]
    fn middle_of_fraction() {
        let a_current = PI / 2.0;
        let actual_angle = interpolate_direction(0.0, a_current, 0.0, 0.5);
        assert_float_equal(actual_angle, PI / 2.0);
    }
    #[test]
    fn quarter_of_fraction() {
        let foo = interpolate_direction(0.0, PI / 2.0, PI, 0.25);
        assert_float_equal(foo, PI / 2.0 - PI / 8.0);
    }
    #[test]
    fn three_quarters_of_fraction() {
        let foo = interpolate_direction(0.0, PI / 2.0, PI, 0.75);
        assert_float_equal(foo, PI / 2.0 + PI / 8.0);
    }
    #[test]
    fn full_fraction() {
        let foo = interpolate_direction(0.0, PI / 2.0, PI, 1.0);
        assert_eq!(foo, PI * 3.0 / 4.0);
    }
    #[test]
    fn zero_fraction() {
        let foo = interpolate_direction(0.0, PI / 2.0, PI, 0.0);
        assert_eq!(foo, PI / 4.0);
    }
}
