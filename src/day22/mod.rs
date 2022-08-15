/*
Day 22: Reactor Reboot

The reactor is a large 3D grid. Each point can be on or off.
The initialization process specifies a cuboid of points to turn on/off.
Each step is executed in order. For example:
    on x=10..12,y=10..12,z=10..12
    on x=11..13,y=11..13,z=11..13
    off x=9..11,y=9..11,z=9..11
    on x=10..10,y=10..10,z=10..10
The above would result in 39 points being turned on at the end.

Part 1: Given the initialization instructions, consider only points within -50,50 for all dimensions.
How many points are on at the end of the instructions?

Part 2: How many points are on when considering all instructions?
*/

use std::collections::HashSet;
use std::cmp;
use std::fs;

#[derive(Debug, Clone)]
pub struct Step {
    on: bool,
    cuboid: Cuboid
}

#[derive(Debug, Clone)]
pub struct Cuboid {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32
}

impl Cuboid {
    // Attempts to create a new cuboid
    // returns None if the dimensions are invalid
    fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32, z_min: i32, z_max: i32) -> Option<Self> {
        if x_min > x_max || y_min > y_max || z_min > z_max {
            return None;
        }
        Some(Cuboid { x_min, x_max, y_min, y_max, z_min, z_max })
    }

    // Ranges are inclusive, an x range of 1 to 4 has a length of 4 (not 3)
    // so we add 1 to each dimention to accurately calculate volume
    fn volume(&self) -> usize {
        (self.x_max - self.x_min + 1) as usize
        * (self.y_max - self.y_min + 1) as usize
        * (self.z_max - self.z_min + 1) as usize
    }

    // two cuboids intersect with each other if, for each dimension,
    // the smallest maximum point is greater than the largest minimum point.
    // Inspired by https://stackoverflow.com/a/5556796
    fn intersects(&self, other: &Cuboid) -> bool {
        return cmp::min(self.x_max, other.x_max) >= cmp::max(self.x_min, other.x_min)
            && cmp::min(self.y_max, other.y_max) >= cmp::max(self.y_min, other.y_min)
            && cmp::min(self.z_max, other.z_max) >= cmp::max(self.z_min, other.z_min);
    }

    // Given two cuboids, subtract the intersecting area of the other cube from self
    // then return a list of cuboids comprising the remaining area of what used to be self.
    // If the two cuboids do not intersect, just return a vector containing self.
    // This splits self up along each possible intersecting dimension, a total of 6 possible slices.
    // Not all slices will be valid, depending on how the two cuboids intersect;
    // invalid slices are filtered out of the vector
    fn subtract(&self, other: &Cuboid) -> Vec<Cuboid> {
        if !self.intersects(other) {
            return vec![self.to_owned()];
        }
        [
            Cuboid::new(
                self.x_min,
                other.x_min - 1,
                self.y_min,
                self.y_max,
                self.z_min,
                self.z_max
            ),
            Cuboid::new(
                other.x_max + 1,
                self.x_max,
                self.y_min,
                self.y_max,
                self.z_min,
                self.z_max
            ),
            Cuboid::new(
                cmp::max(self.x_min, other.x_min),
                cmp::min(self.x_max, other.x_max),
                self.y_min,
                other.y_min -1,
                self.z_min,
                self.z_max
            ),
            Cuboid::new(
                cmp::max(self.x_min, other.x_min),
                cmp::min(self.x_max, other.x_max),
                other.y_max + 1,
                self.y_max,
                self.z_min,
                self.z_max
            ),
            Cuboid::new(
                cmp::max(self.x_min, other.x_min),
                cmp::min(self.x_max, other.x_max),
                cmp::max(self.y_min, other.y_min),
                cmp::min(self.y_max, other.y_max),
                self.z_min,
                other.z_min - 1
            ),
            Cuboid::new(
                cmp::max(self.x_min, other.x_min),
                cmp::min(self.x_max, other.x_max),
                cmp::max(self.y_min, other.y_min),
                cmp::min(self.y_max, other.y_max),
                other.z_max + 1,
                self.z_max
            ),
        ]
        .into_iter()
        .filter_map(|c| c)
        .collect()
    }
}

// Part 1: brute force
// runs in about 1.5 seconds
pub fn cubes_on_50(steps: &Vec<Step>) -> usize {
    let filtered_steps: Vec<_> = steps.into_iter().filter(|step| 
        step.cuboid.x_min >= -50 && step.cuboid.x_max <= 50 && 
        step.cuboid.y_min >= -50 && step.cuboid.y_max <= 50 &&
        step.cuboid.z_min >= -50 && step.cuboid.z_max <= 50
    ).collect();
    // use a set to represent grid spaces that are on
    let mut on: HashSet<(i32,i32,i32)> = HashSet::new();
    for step in filtered_steps {
        for x in step.cuboid.x_min..=step.cuboid.x_max {
            for y in step.cuboid.y_min..=step.cuboid.y_max {
                for z in step.cuboid.z_min..=step.cuboid.z_max {
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

// Part 2
// Keep a list of cuboids that are in the On position
// for each step
//      Compare the cuboid of the new step to the existing list of cuboids
//          If they intersect, split the existing one into component cuboids *that don't intersect*
//          If the step is "on", add the new cuboid
// Add up the volumes of the list on cuboids to determine the number of "on" spaces
pub fn all_cubes_on(steps: &Vec<Step>) -> usize {
    let mut on_cuboids: Vec<Cuboid> = Vec::new();

    for step in steps {
        let mut sliced_cuboids: Vec<Cuboid> = Vec::new();
        for existing in on_cuboids {
            sliced_cuboids.extend(existing.subtract(&step.cuboid));
        }
        on_cuboids = sliced_cuboids;
        if step.on {
            on_cuboids.push(step.cuboid.clone());
        }
    }

    on_cuboids.into_iter()
        .map(|c| c.volume())
        .sum()
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
        cuboid: Cuboid {
            x_min: coords[0][0],
            x_max: coords[0][1],
            y_min: coords[1][0],
            y_max: coords[1][1],
            z_min: coords[2][0],
            z_max: coords[2][1]
        }
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
            on x=-41..9,y=-7..43,z=-33..15";
        parse_input(input)
    }

    #[test]
    fn test_count_on_50() {
        let test_data = get_test_data();
        assert_eq!(590784, cubes_on_50(&test_data));
    }

    #[test]
    fn test_intersects() {
        let c1 = Cuboid::new(0, 10, 0, 10, 0, 10).unwrap();
        let c2 = Cuboid::new(5, 20, -5, 5, 5, 10).unwrap();
        assert_eq!(true, c1.intersects(&c2));
        let c3 = Cuboid::new(5, 20, -5, 5, 20, 50).unwrap();
        assert_eq!(false, c1.intersects(&c3));
    }

    #[test]
    fn test_verify_count_50_intersect() {
        let test_data = get_test_data();
        assert_eq!(590784, all_cubes_on(&test_data));
    }

    #[test]
    fn test_large_initialization_cube_input() {
        let input = "on x=-5..47,y=-31..22,z=-19..33
            on x=-44..5,y=-27..21,z=-14..35
            on x=-49..-1,y=-11..42,z=-10..38
            on x=-20..34,y=-40..6,z=-44..1
            off x=26..39,y=40..50,z=-2..11
            on x=-41..5,y=-41..6,z=-36..8
            off x=-43..-33,y=-45..-28,z=7..25
            on x=-33..15,y=-32..19,z=-34..11
            off x=35..47,y=-46..-34,z=-11..5
            on x=-14..36,y=-6..44,z=-16..29
            on x=-57795..-6158,y=29564..72030,z=20435..90618
            on x=36731..105352,y=-21140..28532,z=16094..90401
            on x=30999..107136,y=-53464..15513,z=8553..71215
            on x=13528..83982,y=-99403..-27377,z=-24141..23996
            on x=-72682..-12347,y=18159..111354,z=7391..80950
            on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
            on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
            on x=-52752..22273,y=-49450..9096,z=54442..119054
            on x=-29982..40483,y=-108474..-28371,z=-24328..38471
            on x=-4958..62750,y=40422..118853,z=-7672..65583
            on x=55694..108686,y=-43367..46958,z=-26781..48729
            on x=-98497..-18186,y=-63569..3412,z=1232..88485
            on x=-726..56291,y=-62629..13224,z=18033..85226
            on x=-110886..-34664,y=-81338..-8658,z=8914..63723
            on x=-55829..24974,y=-16897..54165,z=-121762..-28058
            on x=-65152..-11147,y=22489..91432,z=-58782..1780
            on x=-120100..-32970,y=-46592..27473,z=-11695..61039
            on x=-18631..37533,y=-124565..-50804,z=-35667..28308
            on x=-57817..18248,y=49321..117703,z=5745..55881
            on x=14781..98692,y=-1341..70827,z=15753..70151
            on x=-34419..55919,y=-19626..40991,z=39015..114138
            on x=-60785..11593,y=-56135..2999,z=-95368..-26915
            on x=-32178..58085,y=17647..101866,z=-91405..-8878
            on x=-53655..12091,y=50097..105568,z=-75335..-4862
            on x=-111166..-40997,y=-71714..2688,z=5609..50954
            on x=-16602..70118,y=-98693..-44401,z=5197..76897
            on x=16383..101554,y=4615..83635,z=-44907..18747
            off x=-95822..-15171,y=-19987..48940,z=10804..104439
            on x=-89813..-14614,y=16069..88491,z=-3297..45228
            on x=41075..99376,y=-20427..49978,z=-52012..13762
            on x=-21330..50085,y=-17944..62733,z=-112280..-30197
            on x=-16478..35915,y=36008..118594,z=-7885..47086
            off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
            off x=2032..69770,y=-71013..4824,z=7471..94418
            on x=43670..120875,y=-42068..12382,z=-24787..38892
            off x=37514..111226,y=-45862..25743,z=-16714..54663
            off x=25699..97951,y=-30668..59918,z=-15349..69697
            off x=-44271..17935,y=-9516..60759,z=49131..112598
            on x=-61695..-5813,y=40978..94975,z=8655..80240
            off x=-101086..-9439,y=-7088..67543,z=33935..83858
            off x=18020..114017,y=-48931..32606,z=21474..89843
            off x=-77139..10506,y=-89994..-18797,z=-80..59318
            off x=8476..79288,y=-75520..11602,z=-96624..-24783
            on x=-47488..-1262,y=24338..100707,z=16292..72967
            off x=-84341..13987,y=2429..92914,z=-90671..-1318
            off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
            off x=-27365..46395,y=31009..98017,z=15428..76570
            off x=-70369..-16548,y=22648..78696,z=-1892..86821
            on x=-53470..21291,y=-120233..-33476,z=-44150..38147
            off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";
        let test_data = parse_input(input);
        assert_eq!(2758514936282235, all_cubes_on(&test_data));
    }
}
