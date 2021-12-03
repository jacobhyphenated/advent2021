use std::env;
use std::process;

mod codewars;
mod day3_old;
mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: list each day you want to run:");
        println!("    example:");
        println!("    advent day1 day15");
        process::exit(0);
    }
    let days = &args[1..];
    for day in days {
        if day == "codewars" {

        }
        if day == "day1" {
            let depths = day1::read_depths();
            println!("Part 1: {} increases", day1::count_increases(&depths));
            println!("Part 2: {} increases using 3 value rolling average", day1::count_rolling2(&depths));
        }
        if day == "day2" {
            let commands = day2::read_commands();
            println!("Part 1: Depth x Position = {}", day2::calc_position(&commands));
            println!("Part 2: Position using Aim = {}", day2::calc_aim(&commands));
        }
        if day == "day3" {
            let diag = day3::read_diagnostic();
            println!("Part 1: Power = {}", day3::power(&diag));
        }
    }
}
