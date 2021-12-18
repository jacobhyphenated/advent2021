/*
Day 13: Transparent Origami

Fold a piece of transparent paper in half horizontally or vertically.
Dots on the paper then overlap or combine.
All folds occur at exact half way points, the folded line is discarded
and there are an equal number of lines on either side of the fold.

Part 1: do the first fold, then count the number of dots

Part 2: do all the folds, the dots spell out a message in capital letters.
*/

use std::fs;

// Part 1 - do a single fold (instruction), then count the "dots"
// which are the number of "true" values in the 2d array
pub fn dots_one_fold(dots: &Vec<Vec<bool>>, instruction: &str) -> usize {
    fold(dots, instruction).iter()
        .flat_map(|line| line)
        .filter(|&val| *val)
        .count()
}

// Part 2 - iterate through the fold instructions, replacing the "dots" after each step
// just return the 2d array and eyeball it - no idea how to do this part programatically
pub fn fold_all(dots: &Vec<Vec<bool>>, instructions: &Vec<String>) -> Vec<Vec<bool>> {
    instructions.iter().fold(dots.clone(), |dots, instruction| fold(&dots, instruction))
}

// Use different methods for horizontal vs vertical folds
fn fold(dots: &Vec<Vec<bool>>, instruction: &str) -> Vec<Vec<bool>> {
    let parts: Vec<_> = instruction.trim().split("=").collect();
    let index = parts[1].parse().unwrap();
    match parts[0] {
        "fold along y" => fold_horizontal(dots, index),
        "fold along x" => fold_vertical(dots, index),
        _ => panic!("Bad fold request {}", parts[0])
    }

}

fn fold_horizontal(dots: &Vec<Vec<bool>>, index: usize) -> Vec<Vec<bool>> {
    let top = &dots[..index];
    let bottom = &dots[index+1..dots.len()];
    let mut result = vec![vec![false; top[0].len()]; top.len()];
    for row in 0..index {
        for col in 0..dots[row].len() {
            // count up from the top and count down for the bottom slice, meet in the middle
            result[row][col] = top[row][col] || bottom[bottom.len() - 1 - row][col];
        }
    }
    return result;
}

fn fold_vertical(dots: &Vec<Vec<bool>>, index: usize) -> Vec<Vec<bool>> {
    let left: Vec<_> = dots.iter().map(|row| &row[..index]).collect();
    let right: Vec<_> = dots.iter().map(|row| &row[index+1..]).collect();
    let mut result = vec![vec![false; left[0].len()]; left.len()];
    for row in 0..dots.len() {
        for col in 0..index {
            result[row][col] = left[row][col] || right[row][right[row].len() - 1 - col];
        }
    }
    return result;
}

pub fn read_data() -> (Vec<Vec<bool>>, Vec<String>) {
    let dots = fs::read_to_string("src/day13/dots.txt").expect("missing dots.txt");
    let instructions = fs::read_to_string("src/day13/folds.txt").expect("missing folds.txt");

    let instructions: Vec<String> = instructions.lines().map(|line| line.trim().to_string()).collect();
    (parse_dots(&dots), instructions)
}

fn parse_dots(input: &str) -> Vec<Vec<bool>> {
    let mut largest_x = 0;
    let mut largets_y = 0;
    let mut points: Vec<(usize, usize)> = Vec::new();
    for line in input.lines() {
        let p: Vec<_> = line.trim().split(",").map(|point| point.parse::<usize>().unwrap()).collect();
        if p[0] > largest_x {
            largest_x = p[0];
        }
        if p[1] > largets_y {
            largets_y = p[1];
        }
        points.push((p[0], p[1]))
    }

    let mut dots = vec![vec![false; largest_x + 1]; largets_y + 1];
    for (x, y) in points {
        dots[y][x] = true;
    }

    dots
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_dots() -> Vec<Vec<bool>> {
        let input = "6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0";
        parse_dots(input)
    }

    #[test]
    fn test_dots() {
        let dots = get_dots();
        assert_eq!(true, dots[3][0]);
        assert_eq!(true, dots[10][1]);
        assert_eq!(true, dots[10][6]);
        assert_eq!(false, dots[10][7]);
    }

    #[test]
    fn test_one_fold() {
        let dots = get_dots();
        assert_eq!(17, dots_one_fold(&dots, "fold along y=7"));
    }

    #[test]
    fn test_two_folds() {
        let dots = get_dots();
        let dots = fold(&dots, "fold along y=7");
        assert_eq!(16, dots_one_fold(&dots, "fold along x=5"))
    }
}