/*
Day 19: Beacon Scanner

A bunch of beacons are dropped into the water to map the ocean.
A bunch of scanners pick up the beacons closest to them.
However, the scanners only know the becons position relative to the scanner.
The scanner can be oriented in any one of 24 possible rotations and flips in 3d space.

Use Scanner 0 as the frame of reference for the other scanners.
You can tell when scanners pick up the same beacons if there are 12 or more overlapping beacons.
Find where the beacons overlap and orient them and their scanner relative to scanner 0.

Part 1: How many beacons are there?

Part 2: What is the manhattan distance of the scanners that are the farthest apart?
*/

use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    // euclidean distance is a float. Truncate to i32 to avoid potential FP issues
    // and to just be easier to deal with in general.
    fn distance(&self, other: &Point) -> i32 {
        f32::sqrt(((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f32) as i32
    }

    fn translate(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    fn manhattan(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

// Parts 1 and 2. Not the cleanest solution, and takes around 22 seconds to run.
// Brute force each possible rotation of each scanner compared to a set of known beacon positions.
pub fn locate_beacons(scanners: &Vec<Vec<Point>>) -> (usize, i32) {
    // Start with Scanner 0 as the reference beacons - store in a set of known beacons
    let mut known_beacons: HashSet<Point> = scanners[0].iter().map(|p| p.clone()).collect();
    let mut known_scanners = vec![Point::new(0,0,0)];
    // Other scanners are marked as unknown
    let mut unknown_scanners: Vec<usize> = (1..scanners.len()).collect();
    // compare unknown scanners to known beacon positions until all scanners are known
    while unknown_scanners.len() > 0 {
        for &i in &unknown_scanners {
            // Check if we can determine the position of this scanner
            if let Some((scanner, beacons)) = determine_scanner_location(&scanners[i], &known_beacons) {
                known_scanners.push(scanner);
                for p in beacons {
                    known_beacons.insert(p);
                }
                unknown_scanners.retain(|&index| index != i);
                break;
            }
        }
    }

    // Once all beacons and scanners are oriented around scanner 0
    // we search for the manhattan distance for part 2
    let mut farthest = 0;
    for i in 0..known_scanners.len() - 1 {
        for j in 1..known_scanners.len() {
            let manhattan_distance = known_scanners[i].manhattan(&known_scanners[j]);
            if manhattan_distance > farthest {
                farthest = manhattan_distance;
            }
        }
    }
    return (known_beacons.len(), farthest);
}

/*
loop through rotations
    compute distance for every point in scanner to every known beacon
    store in a HashMap: distance -> vec<(point,point)>
    for all counts of any map key is >= 12 (at least 12 points have the same distance to a known point)
        attempt to find translation value that matches scanner point with known point
        apply translation value to all points, if > 12 match, we have a winner
            apply the rotation and translation to all beacons in the scanner
            scanner position is the translation (relative to 0,0,0)
*/
fn determine_scanner_location(scanner: &Vec<Point>, known_points: &HashSet<Point>) -> Option<(Point, Vec<Point>)> {
    for rotation in 1..=24 {
        let rotated_points: Vec<_> = scanner.iter().map(|p| rotate(&p, rotation)).collect();
        let mut distance_map: HashMap<i32, Vec<(&Point, &Point)>> = HashMap::new();
        for p in &rotated_points {
            for known in known_points {
                let distance = p.distance(known);
                let list = distance_map.entry(distance).or_insert(vec![]);
                list.push((p, known));
            }
        }
        for (_, possible_translation) in distance_map.iter().filter(|(_,v)| v.len() >= 12){
            for &pair in possible_translation {
                let translation = Point::new(pair.1.x - pair.0.x, pair.1.y - pair.0.y, pair.1.z - pair.0.z);
                let mut match_count = 0;
                for &p in possible_translation {
                    if p.0.translate(&translation) == *p.1 {
                        match_count += 1;
                    }
                }
                if match_count >= 12 {
                    let translated: Vec<Point> = rotated_points.iter()
                        .map(|beacon| beacon.translate(&translation))
                        .collect();
                    return Some((translation, translated));
                }
            }
        }
    }
    None
}

// computed these by hand by taking a cube, writing x,y,z,-x,-y,-z on the sides
// then rotating it in all possible directions until we had 24 states
// (would have been easier to just do all 48 possible orientations)
fn rotate(p: &Point, rotation: i32) -> Point {
    match rotation {
        1 => Point::new(p.x, p.y, p.z),
        2 => Point::new(p.x, -p.z, p.y),
        3 => Point::new(p.x, -p.y, -p.z),
        4 => Point::new(p.x, p.z, -p.y),
        5 => Point::new(p.z, p.y, -p.x),
        6 => Point::new(p.z, p.x, p.y),
        7 => Point::new(p.z, -p.y, p.x),
        8 => Point::new(p.z, -p.x, -p.y),
        9 => Point::new(-p.x, p.y, -p.z),
        10 => Point::new(-p.x, p.z, p.y),
        11 => Point::new(-p.x, -p.y, p.z),
        12 => Point::new(-p.x, -p.z, -p.y),
        13 => Point::new(p.y, -p.x, p.z),
        14 => Point::new(p.y, -p.z, -p.x),
        15 => Point::new(p.y, p.x, -p.z),
        16 => Point::new(p.y, p.z, p.x),
        17 => Point::new(-p.y, p.z, -p.x),
        18 => Point::new(-p.y, p.x, p.z),
        19 => Point::new(-p.y, -p.z, p.x),
        20 => Point::new(-p.y, -p.x, -p.z),
        21 => Point::new(-p.z, p.y, p.x),
        22 => Point::new(-p.z, -p.x, p.y),
        23 => Point::new(-p.z, -p.y, -p.x),
        24 => Point::new(-p.z, p.x, -p.y),
        _ => panic!("invalid rotation")
    }
}

fn parse_input(input: &str) -> Vec<Vec<Point>> {
    input.split("\n\n")
        .map(|scanner| {
            let mut lines = scanner.lines();
            lines.next(); // skip --- scanner --- line
            lines.map(|line| {
                let p: Vec<i32> = line.trim().split(",").map(|p| p.parse().unwrap()).collect();
                Point::new(p[0], p[1], p[2])
            })
            .collect()
        })
        .collect()
}

pub fn read_input() -> Vec<Vec<Point>> {
    let input = fs::read_to_string("src/day19/scanners.txt").expect("missing scanners.txt");
    parse_input(&input)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_scanner_data() -> Vec<Vec<Point>> {
        let input = "--- scanner 0 ---
            404,-588,-901
            528,-643,409
            -838,591,734
            390,-675,-793
            -537,-823,-458
            -485,-357,347
            -345,-311,381
            -661,-816,-575
            -876,649,763
            -618,-824,-621
            553,345,-567
            474,580,667
            -447,-329,318
            -584,868,-557
            544,-627,-890
            564,392,-477
            455,729,728
            -892,524,684
            -689,845,-530
            423,-701,434
            7,-33,-71
            630,319,-379
            443,580,662
            -789,900,-551
            459,-707,401

            --- scanner 1 ---
            686,422,578
            605,423,415
            515,917,-361
            -336,658,858
            95,138,22
            -476,619,847
            -340,-569,-846
            567,-361,727
            -460,603,-452
            669,-402,600
            729,430,532
            -500,-761,534
            -322,571,750
            -466,-666,-811
            -429,-592,574
            -355,545,-477
            703,-491,-529
            -328,-685,520
            413,935,-424
            -391,539,-444
            586,-435,557
            -364,-763,-893
            807,-499,-711
            755,-354,-619
            553,889,-390

            --- scanner 2 ---
            649,640,665
            682,-795,504
            -784,533,-524
            -644,584,-595
            -588,-843,648
            -30,6,44
            -674,560,763
            500,723,-460
            609,671,-379
            -555,-800,653
            -675,-892,-343
            697,-426,-610
            578,704,681
            493,664,-388
            -671,-858,530
            -667,343,800
            571,-461,-707
            -138,-166,112
            -889,563,-600
            646,-828,498
            640,759,510
            -630,509,768
            -681,-892,-333
            673,-379,-804
            -742,-814,-386
            577,-820,562

            --- scanner 3 ---
            -589,542,597
            605,-692,669
            -500,565,-823
            -660,373,557
            -458,-679,-417
            -488,449,543
            -626,468,-788
            338,-750,-386
            528,-832,-391
            562,-778,733
            -938,-730,414
            543,643,-506
            -524,371,-870
            407,773,750
            -104,29,83
            378,-903,-323
            -778,-728,485
            426,699,580
            -438,-605,-362
            -469,-447,-387
            509,732,623
            647,635,-688
            -868,-804,481
            614,-800,639
            595,780,-596

            --- scanner 4 ---
            727,592,562
            -293,-554,779
            441,611,-461
            -714,465,-776
            -743,427,-804
            -660,-479,-426
            832,-632,460
            927,-485,-438
            408,393,-506
            466,436,-512
            110,16,151
            -258,-428,682
            -393,719,612
            -211,-452,876
            808,-476,-593
            -575,615,604
            -485,667,467
            -680,325,-822
            -627,-443,-432
            872,-547,-609
            833,512,582
            807,604,487
            839,-516,451
            891,-625,532
            -652,-548,-490
            30,-46,-14";
        parse_input(input)
    }
    
    #[test]
    fn test_locate_beacons() {
        let scanners = get_scanner_data();
        let (beacons, farthest) = locate_beacons(&scanners);
        assert_eq!(79, beacons);
        assert_eq!(3621, farthest);
    }
}


