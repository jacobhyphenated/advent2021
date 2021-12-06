/*
Day 5: Hydrothermal Venture

Puzzle input is a list of line segments like "9,4 -> 3,4".
Some of these line segments overlap each other.

Part 1: Using only straight horizontal or straight vertical lines,
count the number of points with multiple "vents"

Part 2: Now use all lines including diagonals.
All diaganals in the puzzle are 45 degree angled slopes.
*/

use std::collections::HashMap;
use std::cmp;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq)]
pub struct LineSegment {
    p1: Point,
    p2: Point
}

/**
 * Part 1. Not super elegent
 *      Split input into horizontal and vertical lines
 *      Iterate over points in the lines by incrementing the x or y value
 *      use a map with the "Point" as the key to count occurences of that point.
 */
pub fn count_straight_overlaps(lines: &Vec<LineSegment>) -> usize {
    let horizontal_lines: Vec<_> = lines.iter().filter(|ls| ls.p1.y == ls.p2.y).collect();
    let vertical_lines: Vec<_> = lines.iter().filter(|ls| ls.p1.x == ls.p2.x).collect();
    let mut grid: HashMap<Point, usize> = HashMap::new();
    for ls in horizontal_lines {
        for x in cmp::min(ls.p1.x, ls.p2.x)..=cmp::max(ls.p1.x, ls.p2.x) {
            let point = Point {x: x, y: ls.p1.y};
            *grid.entry(point).or_insert(0) += 1;
        }
    }
    for ls in vertical_lines {
        for y in cmp::min(ls.p1.y, ls.p2.y)..=cmp::max(ls.p1.y, ls.p2.y) {
            let point = Point {x: ls.p1.x, y: y};
            *grid.entry(point).or_insert(0) += 1;
        }
    }
    grid.iter().filter(|(_, &count)| count > 1).count()
}

/**
 * Part 2
 * Struggled accomplishing some of the ideas I had.
 * Tried to do range iterators, but couldn't get the types to work right (range and range.rev() are different types)
 * Ownership with `current` and the grid were tricky to, making the while loop awkward.
 * 
 *      Loop through all line segments
 *          Find the next point by incrementing or decrenting x and y if necessary
 *          (seems like there should be a more elegent way to do this)
 *          Finished the line when the next point is the end point defined in the LineSegment
 *      Use the same concept of the grid HashMap as in part1
 */ 
pub fn count_all_overlaps(lines: &Vec<LineSegment>) -> usize {
    let mut grid: HashMap<Point, usize> = HashMap::new();
    for ls in lines {
        let mut current = Point { x: ls.p1.x, y: ls.p1.y };
        while current != ls.p2 {
            let next_x = 
                if ls.p1.x == ls.p2.x {
                    current.x
                } else if ls.p1.x < ls.p2.x {
                    current.x + 1
                } else {
                    current.x - 1
                };
            let next_y = 
                if ls.p1.y == ls.p2.y {
                    current.y
                } else if ls.p1.y < ls.p2.y {
                    current.y + 1
                } else {
                    current.y - 1
                };
            *grid.entry(current).or_insert(0) += 1;

            current = Point { x: next_x, y: next_y};
        }
        *grid.entry(current).or_insert(0) += 1;
    }

    grid.iter().filter(|(_, &count)| count > 1).count()
}

pub fn read_data() -> Vec<LineSegment> {
    let input = fs::read_to_string("src/day5/lines.txt").expect("missing lines.txt");
    parse_data(&input[..])
}

fn parse_data(data: &str) -> Vec<LineSegment> {
    data.lines().map(|line| {
        let points: Vec<_> = line.trim().split(" -> ").collect();
        let mut points = points.into_iter()
            .map(|p| p.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>())
            .map(|point| Point { x: point[0], y: point[1]})
            .into_iter();
        // Mem ownership - need to use into_iter to move ownership, otherwise must clone()
        LineSegment { p1: points.next().unwrap(), p2: points.next().unwrap()}
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<LineSegment> {
        let data = "0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2";
        parse_data(data)
    }

    #[test]
    fn test_straight_overlaps() {
        let lines = test_data();
        assert_eq!(5, count_straight_overlaps(&lines));
    }

    #[test]
    fn test_all_overlaps() {
        let lines = test_data();
        assert_eq!(12, count_all_overlaps(&lines));
    }
}
