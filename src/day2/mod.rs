/*
Day 2: Dive!

The puzzle input is a list of commands "forward", "down", and "up" followed by an integer.

Part 1: "forward" adds to the horizontal position, "down" increases depth, "up" decreases depth.
Find the product of the position and the depth after processing all commands.

Part 2: There is a third component called aim. "down X" increases aim by X. "up X" decreases aim by X.
The "forward X" command increases horizontal position by X AND increases depth by aim times X.
*/

use std::fs;

pub fn calc_position(commands: &Vec<String>) -> i32 {
    let mut position = (0, 0);
    for command in commands {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let value: i32 = parts[1].parse().unwrap();
        let (x,y) = position;
        position = match parts[0] {
            "forward"   => (x + value, y),
            "down"      => (x, y + value),
            "up"        => (x, y - value),
            _           => (x, y)
        }
    }
    return position.0 * position.1;
}

#[derive(Debug)]
struct Heading {
    aim: i64,
    position: i64,
    depth: i64
}

pub fn calc_aim(commands: &Vec<String>) -> i64 {
    let mut heading = Heading { aim: 0, position: 0, depth: 0 };
    for command in commands {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let value: i64 = parts[1].parse().unwrap();
        heading = match parts[0] {
            "forward" => Heading { aim: heading.aim, position: heading.position + value, depth: heading.depth + heading.aim * value },
            "down" => Heading { aim: heading.aim + value, position: heading.position, depth: heading.depth },
            "up" => Heading { aim: heading.aim - value, position: heading.position, depth: heading.depth },
            _ => heading
        }
    }
    return heading.position * heading.depth;
}

pub fn read_commands() -> Vec<String> {
    let file = fs::read_to_string("src/day2/commands.txt").expect("file commands.txt not found");
    file.lines().map(|line| line.trim().to_string()).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_position() {
        let commands = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]
            .iter().map(|c| c.to_string()).collect();
        assert_eq!(150, calc_position(&commands));
    }

    #[test]
    fn test_calc_aim() {
        let commands = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]
            .iter().map(|c| c.to_string()).collect();
        assert_eq!(900, calc_aim(&commands));
    }
}