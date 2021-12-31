/*
Day 20: Trench Map

You have 2 parts, an image enhancement algorithm, and an image map.
The image map is a 2d grid. To enhance each point in the grid:
    look at the surrounding points (9 points total)
    Convert them to binary. For example:
    ...
    #..
    .#.
    ...#...#. == 000100010 == 34
    look up the 34th character in the enhancement algorithm to get the result for that point.
All points are process simultaneously
The output grid is infinute expanding in all directions

Part 1: how many points are "on" in the '#' position after 2 steps
Part 2: how many points are on after 50 steps

*/

use std::fs;

// Parts 1 & 2 - just change the number of steps
// part 2 runs ~4 seconds
// The trick with the infinite grid is the first and last char in the enhance array
// in the sample, both are '.' so we can pad out '.' or 'false' on our output grid.
// but in the puzzle input, ehnance[0] == '#'. Which means that a grid of 9 falses evaluates to true.
// Also, a grid on 9 trues evaluates to false. This means the infinite padding flips from true/false every step.
// Solve this by considering only the raw input grid + 1 padded row/col in each direction for each step
// the padding changes from true/false each step if the enhance vector is true in the 0 place.
// For each step, expand our search area by one row and one column in all directions. 
pub fn count_after_steps(image: &Vec<Vec<bool>>, enhance: &Vec<bool>, steps: usize) -> usize {
    let mut pad = enhance[0];
    let mut pad_len = steps;
    let mut enhanced = pad_grid(image, steps);
    for _ in 0..steps {
        enhanced = apply_enhancement(&enhanced, enhance, pad, pad_len);
        pad = if enhance[0] { !pad } else { pad };
        pad_len -= 1;
    }
    enhanced.iter().flat_map(|col| col.iter().filter(|&v| *v).collect::<Vec<_>>()).count()
}

// pad specifies if the outer infinity padding should be true or false for this step
// pad_len narrows the range we actually search and evaluate for our enhancement steps
fn apply_enhancement(image: &Vec<Vec<bool>>, enhance: &Vec<bool>, pad: bool, pad_len: usize) -> Vec<Vec<bool>> {
    let mut result = vec![vec![pad; image[0].len()]; image.len()];
    for r in pad_len..image.len() - pad_len {
        for c in pad_len..image[r].len() - pad_len {
            result[r][c] = enhance[find_surrounding(r, c, image)];
        }
    }
    result
}

fn find_surrounding(row: usize, col: usize, image: &Vec<Vec<bool>>) -> usize {
    let mut adjacent = Vec::new();
    for r in row-1..=row+1 {
        for c in col-1..=col+1 {
            // get a '1' or '0' bit character
            adjacent.push(if image[r][c] { '1' } else { '0' });
        }
    }
    let binary: String = adjacent.iter().collect();
    usize::from_str_radix(&binary, 2).unwrap()
}

// Pad the input grid exactly enough for the number of steps we have to run
fn pad_grid(image: &Vec<Vec<bool>>, steps: usize) -> Vec<Vec<bool>> {
    let pad = (steps+1) * 2;
    let mut padded = vec![vec![false; image[0].len() + pad]; image.len() + pad];
    image.iter().enumerate()
        .flat_map(|(row, val)| val.iter().enumerate().map(move |(col, v)| (row,col,v)))
        .for_each(|(r,c,v)| {
            padded[r+steps+1][c+steps+1] = *v;
        });
    padded
}

fn parse_enhancement_algo(input: &str) -> Vec<bool> {
    input.chars().map(|c|{
        match c {
            '#' => true,
            _ => false
        }
    }).collect()
}

fn parse_input_image(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line| line.trim()
        .chars().map(|c| {
            match c {
                '#' => true,
                _ => false
            }
        }).collect()
    ).collect()
}

pub fn read_data() -> (Vec<Vec<bool>>, Vec<bool>) {
    let image = fs::read_to_string("src/day20/image.txt").expect("missing image.txt");
    let enhance = fs::read_to_string("src/day20/enhance.txt").expect("missing enhance.txt");
    (parse_input_image(&image), parse_enhancement_algo(&enhance))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Vec<bool>> {
        let input = 
            "#..#.
            #....
            ##..#
            ..#..
            ..###";
        parse_input_image(input)
    }

    fn get_enhancement() -> Vec<bool> {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";
        parse_enhancement_algo(input)
    }

    #[test]
    fn test_surrouding_number() {
        let enhance = get_enhancement();
        let image = get_input();
        let surrounding = find_surrounding(2, 2, &image);
        assert_eq!(34, surrounding);
        assert_eq!(true, enhance[surrounding]);
    }

    #[test]
    fn test_image_enhance() {
        let enhance = get_enhancement();
        let image = get_input();
        assert_eq!(35, count_after_steps(&image, &enhance, 2));
        assert_eq!(3351, count_after_steps(&image, &enhance, 50));
    }

}


