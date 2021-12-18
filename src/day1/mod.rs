/*
    Day 1: Sonar Sweep

    Each number in a list is a measurement of depth going further out from the submarine.

    Part 1: Count the number of times the depth increases compared to the previous measurement

    Part 2: Rather than using each individual measurement, count the number of increases
    using a 3 value rolling average.
    Example: [199, 200, 208, 210] would compare 607 to 618 for an increase of 1
*/
use std::fs;

// reduce over a 2 value window/slice of the array
// compare the current value to previous value to increment the accumulator
pub fn count_increases(depths: &Vec<i32>) -> i32 {
    depths.windows(2).fold(0, |increases, slice| {
        if slice[1] > slice[0] { increases + 1} else { increases }
    })
}

pub fn count_rolling(depths: &Vec<i32>) -> i32 {
    let mut increases = 0;
    let mut previous: Option<i32> = None;
    for slice in  depths.windows(3) {
        let current: i32 = slice.iter().sum();
        if current > previous.unwrap_or(current) {
            increases += 1;
        }
        previous = Some(current);
    }
    increases
}

pub fn read_depths() -> Vec<i32> {
    let depths = fs::read_to_string("src/day1/depths.txt").expect("Missing file depths.txt");
    depths.lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, count_increases(&depths));
    }

    #[test]
    fn test_rolling2() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, count_rolling(&depths));
    }
}
