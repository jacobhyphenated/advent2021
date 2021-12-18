/*
Day 17: Trick Shot

We want to send a probe to a target area (a range of x and y coordinates)
We can launch the probe with an initial velocity in the x and y directions
Note: we can launch in positive or negative y, the range is in the negative y
After each step, the x velocity is reduced by one due to water pressure, until it reaches 0
After each step, the y velocity is reduced by one due to gravity (goes less than 0)

Part 1: What is the highest y position possible for any starting velocity that still ends in the target area?

Part 2: How many distinct initial velocity values will put the probe in the target area after any step?
*/

use std::cmp;

#[derive(Debug)]
pub struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl TargetArea {
    fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }
}

// Part 1 is a math, algebra problem
// initial x velocity doesn't matter
    // at a sufficiently high number of steps, x velocity drops to 0
    // there is always a valid value for x at a high number of steps
// Therefore consider only y, on a parabolic trajecotory
// negative velocity of y will equal the initial velocity of y when the y position returns to 0
// so we can maximize the y height with the initial velocity of abs(y_min) - 1
    // Any higher of an initial velocity will always overshoot the target area
// The height after any number of steps, with v = initial velocity, and n = number of steps (0 indexed)
    //n=0 | v
    //n=1 | v + (v - 1)
    //n=2 | v + (v - 1) + (v - 2)
    //n=n | v(n+1) - n(n+1)/2
    // Proof
        // v + v - 1 + v - 2 + ... + v - n
        // (n+1)v - 1 - 2 ... - n
        // (n+1)v - (1 + 2 + .. + n)
        // (n+1)v - (n * (n+1))/2
// The highest point in the parabolic trajecotry is when the number of steps is equal to the initial velocity
// at this point, the velocity is 0
pub fn highest_possible(target: &TargetArea) -> i32 {
    let initial_velocity = target.y_min.abs() - 1;
    let steps = initial_velocity;
    return y_position(initial_velocity, steps);
}

// Part 2 - just brute force it
// dissappointing after all that nice math in part 1
// pick reasonable upper and lower bounds for the initial x and y velocities
// loop through all combonations, and loop through steps to find if the velocity combo is valid
pub fn all_possible_velocities(target: &TargetArea) -> usize {
    let mut valid: Vec<(i32, i32)> = Vec::new();
    // Highest possible valid xv is the max x position of the target area
    // could probably pick a smarter min xv, but this already runs in 12ms
    for xv in 1..=target.x_max {
        // lowest possible y is the bottom of the y target area
        // highest possible y is the same from part 1
        for yv in target.y_min..=(target.y_min.abs() - 1) {
            let mut steps = 0;
            loop {
                let x = x_position(xv, steps);
                let y = y_position(yv, steps);
                if x > target.x_max || y < target.y_min {
                    break;
                }
                if target.is_inside(x, y) {
                    valid.push((x,y));
                    break;
                }
                steps += 1;
            }
        }
    }
    valid.len()
}

fn y_position(initial_velocity: i32, steps: i32) -> i32 {
    return (steps + 1) * initial_velocity - steps * (steps + 1) / 2;
}

fn x_position(initial_velocity: i32, steps: i32) -> i32 {
    let effective_steps = cmp::min(initial_velocity, steps);
    return (effective_steps + 1) * initial_velocity - effective_steps * (effective_steps + 1) / 2;
}

// skip file reading for this one
pub fn read_target_area() -> TargetArea {
    let input = "target area: x=201..230, y=-99..-65";
    let coords: Vec<_> = input.split(": ").collect();
    parse_target_area(coords[1])
}

fn parse_target_area(input: &str) -> TargetArea {
    let split: Vec<_> = input.split(", ").collect();
    let x_range = split[0].split("x=").last().unwrap();
    let x_range: Vec<i32> = x_range.split("..").map(|v| v.parse().unwrap()).collect();

    let y_range = split[1].split("y=").last().unwrap();
    let y_range: Vec<i32> = y_range.split("..").map(|v| v.parse().unwrap()).collect();

    TargetArea {
        x_min: x_range[0],
        x_max: x_range[1],
        y_min: y_range[0],
        y_max: y_range[1]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_y_position() {
        let input = "x=20..30, y=-10..-5";
        let target = parse_target_area(input);
        assert_eq!(45, highest_possible(&target));
    }

    #[test]
    fn test_all_valid_velocities() {
        let input = "x=20..30, y=-10..-5";
        let target = parse_target_area(input);
        assert_eq!(112, all_possible_velocities(&target));
    }

    #[test]
    fn test_target_area() {
        let input = "x=20..30, y=-10..-5";
        let target = parse_target_area(input);
        assert_eq!(20, target.x_min);
        assert_eq!(-10, target.y_min);
        assert_eq!(-5, target.y_max);

        assert_eq!(true, target.is_inside(25, -7));
    }

    #[test]
    fn test_xy_position() {
        assert_eq!(6, x_position(6, 0));
        assert_eq!(11, x_position(6, 1));
        assert_eq!(18, x_position(6, 3));
        assert_eq!(21, x_position(6, 5));
        assert_eq!(21, x_position(6, 10));

        assert_eq!(-9, y_position(3, 8));
    }
}
