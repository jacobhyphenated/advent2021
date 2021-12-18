/*
Day 11: Dumbo Octopus

A 2d grid of octopuses (heretofore octopi), each have an energy state.
Every step, that energy state increases by 1.
If the energy state exceeds 9, then the octopus "flashes".
When an octopus flashes, all adjacent octopi (including diagonals) gain 1 energy
No octopus can flash more than once per steps. All flashed octopi reset to 0 at the end of the step.

Part 1: How many flashes happen after 100 steps?

Part 2: What is the first step in which all octopi flash?
*/
use std::collections::HashSet;
use std::cmp;
use std::fs;

// Part 1 - a lot of logic is reused for parts 1 and 2
// go one step at a time, counting the number of flashes each step
pub fn flash_after_steps(octopi: &Vec<Vec<i32>>, steps: i32) -> i32 {
    let mut octopi = octopi.clone();
    let mut flashes = 0;
    for _ in 0..steps {
        flashes += do_step(&mut octopi).0;
    }
    return flashes;
}

// Part 2
// go one step at a time indefinitely until all octopi flash on the same step
pub fn find_all_flash(octopi: &Vec<Vec<i32>>) -> i32 {
    let mut octopi = octopi.clone();
    let mut step = 1;
    loop {
        if do_step(&mut octopi).1 {
            break;
        }
        step += 1;
    }
    return step;
}

// This function does the work for updating the octopi state each step
// Loop through all octopi
//      add 1 to the energy level
//      call the check_flashes helper method centered on this octopi
// Use a set to track each octopi that flash this step
// once the step is over, reset each flash octopi to 0
// return a tuple - (total number of flashes this step, boolean: true if all octopi flash this step)
fn do_step(octopi: &mut Vec<Vec<i32>>) -> (i32, bool) {
    let mut flashes_this_round: HashSet<(usize, usize)> = HashSet::new();
    let mut flashes = 0;
    for row in 0..octopi.len() {
        for col in 0..octopi[row].len() {
            octopi[row][col] += 1;
            flashes += check_flashes(row, col, octopi, &mut flashes_this_round);
        }
    }

    let all_flash = flashes_this_round.len() == octopi.len() * octopi[0].len();
    // reset flash octopi to 0
    for (r, c) in flashes_this_round {
        octopi[r][c] = 0;
    }

    (flashes, all_flash)
}

// recursive helper function to check for and propogate flashes
// uses a set to track all octopi that have flash this step
// given an octopus, if the energy level is more than 9, and if it hasn't yet flash this step:
//      Add it to the flash set
//      Return flashes equal to 1 + the result of checking flashes on all adjacent octopi
fn check_flashes(row: usize, col: usize, octopi: &mut Vec<Vec<i32>>, flashes_this_round: &mut HashSet<(usize, usize)>) -> i32 {
    if octopi[row][col] > 9 && !flashes_this_round.contains(&(row, col)) {
        flashes_this_round.insert((row,col));
        return 1 + find_adjacent(row, col, &octopi).into_iter()
            .map(|(r, c)| {
                octopi[r][c] += 1;
                check_flashes(r, c, octopi, flashes_this_round)
            })
            .sum::<i32>();
    }
    return 0;
}

// Find adjacent including diagonals
fn find_adjacent(row: usize, col: usize, octopi: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    let max = octopi.len() - 1;
    for r in row.checked_sub(1).unwrap_or(0)..=cmp::min(row + 1, max) {
        let max = octopi[r].len() - 1;
        for c in col.checked_sub(1).unwrap_or(0)..=cmp::min(col + 1, max) {
            if c == col && r == row {
                continue;
            }
            adjacent.push((r, c));
        }
    }
    adjacent
}

pub fn read_octopi() -> Vec<Vec<i32>> {
    let input = fs::read_to_string("src/day11/octopi.txt").expect("mising octopi.txt");
    parse_data(&input)
}

fn parse_data(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| line.trim().chars()
            .map(|c| c.to_string().parse::<i32>().unwrap()).collect()
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Vec<i32>> {
        let test_input = "5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526";
        parse_data(test_input)
    }

    #[test]
    fn test_flashes_after_steps() {
        let octopi = test_data();
        assert_eq!(1656, flash_after_steps(&octopi, 100));
    }

    #[test]
    fn test_all_flash() {
        let octopi = test_data();
        assert_eq!(195, find_all_flash(&octopi));
    }
}
