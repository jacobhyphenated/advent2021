use std::env;
use std::process;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
            println!("Part 2: Life Support = {}", day3::life_support(&diag));
        }
        if day == "day4" {
            let (boards, draws) = day4::read_input();
            println!("Part 1: winning score = {}", day4::first_winner_score(boards.clone(), &draws));
            println!("Part 2: last winner = {}", day4::last_winner_score(boards.clone(), &draws));
        }
        if day == "day5" {
            let lines = day5::read_data();
            let now = Instant::now();
            println!("Part 1: Overlapping Vents (straight lines only) = {}", day5::count_straight_overlaps(&lines));
            println!("Part 1 in {}ms", now.elapsed().as_millis());
            let now = Instant::now();
            println!("Part 2: Overlapping Vents = {}", day5::count_all_overlaps(&lines));
            println!("Part 2 in {}ms", now.elapsed().as_millis());
        }
        if day == "day6" {
            let fish = day6::read_input();
            let now = Instant::now();
            println!("Part 1: total fish (80 days) = {}", day6::calc_growth(&fish, 80));
            println!("Part 1 in {}ms", now.elapsed().as_millis());
            let now = Instant::now();
            println!("Part 2: total fish (256 days) = {}", day6::model_growth(&fish, 256));
            println!("Part 2 in {}ms", now.elapsed().as_millis());
        }
    }
}
