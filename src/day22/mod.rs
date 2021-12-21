
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
pub struct Step {
    on: bool,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32
}

// Part 1: brute force
// runs in about 1.5 seconds
pub fn cubes_on_50(steps: &Vec<Step>) -> usize {
    let filtered_steps: Vec<_> = steps.into_iter().filter(|step| 
        step.x_min >= -50 && step.x_max <= 50 && 
        step.y_min >= -50 && step.y_max <= 50 &&
        step.z_min >= -50 && step.z_max <= 50
    ).collect();
    // use a set to represent grid spaces that are on
    let mut on: HashSet<(i32,i32,i32)> = HashSet::new();
    for step in filtered_steps {
        for x in step.x_min..=step.x_max {
            for y in step.y_min..=step.y_max {
                for z in step.z_min..=step.z_max {
                    if step.on {
                        on.insert((x,y,z));
                    }
                    else {
                        on.remove(&(x,y,z));
                    }
                }
            }
        }
    }    
    on.len()
}

pub fn all_cubes_on(steps: &Vec<Step>) -> usize {
    0
}

fn parse_input(input: &str) -> Vec<Step> {
    input.lines().map(|line| parse_step(line)).collect()
}

fn parse_step(line: &str) -> Step {
    let step: Vec<&str> = line.trim().split(" ").collect();
    let on = match step[0] {
        "on" => true,
        "off" => false,
        _ => panic!("Invalid step command")
    };
    let coords: Vec<Vec<i32>> = step[1].split(",")
        .map(|coord| coord.split("=").last().unwrap())
        .map(|range| range.split("..").map(|val| val.parse().unwrap()).collect())
        .collect();

    Step {
        on,
        x_min: coords[0][0],
        x_max: coords[0][1],
        y_min: coords[1][0],
        y_max: coords[1][1],
        z_min: coords[2][0],
        z_max: coords[2][1]
    }
}

pub fn read_steps() -> Vec<Step> {
    let input = fs::read_to_string("src/day22/steps.txt").expect("missing steps.txt");
    parse_input(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<Step> {
        let input = "on x=-20..26,y=-36..17,z=-47..7
            on x=-20..33,y=-21..23,z=-26..28
            on x=-22..28,y=-29..23,z=-38..16
            on x=-46..7,y=-6..46,z=-50..-1
            on x=-49..1,y=-3..46,z=-24..28
            on x=2..47,y=-22..22,z=-23..27
            on x=-27..23,y=-28..26,z=-21..29
            on x=-39..5,y=-6..47,z=-3..44
            on x=-30..21,y=-8..43,z=-13..34
            on x=-22..26,y=-27..20,z=-29..19
            off x=-48..-32,y=26..41,z=-47..-37
            on x=-12..35,y=6..50,z=-50..-2
            off x=-48..-32,y=-32..-16,z=-15..-5
            on x=-18..26,y=-33..15,z=-7..46
            off x=-40..-22,y=-38..-28,z=23..41
            on x=-16..35,y=-41..10,z=-47..6
            off x=-32..-23,y=11..30,z=-14..3
            on x=-49..-5,y=-3..45,z=-29..18
            off x=18..30,y=-20..-8,z=-3..13
            on x=-41..9,y=-7..43,z=-33..15
            on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
            on x=967..23432,y=45373..81175,z=27513..53682";
        parse_input(input)
    }

    #[test]
    fn test_count_on_50() {
        let test_data = get_test_data();
        assert_eq!(590784, cubes_on_50(&test_data));
    }
}