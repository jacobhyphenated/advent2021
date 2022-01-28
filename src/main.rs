use std::env;
use std::process;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
            println!("Part 2: {} increases using 3 value rolling average", day1::count_rolling(&depths));
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
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            println!("Part 2: total fish (256 days) = {}", day6::model_growth(&fish, 256));
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day7" {
            let subs = day7::read_input();
            let now = Instant::now();
            println!("Part 1: linear gas = {}", day7::linear_gas(&subs));
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            println!("Part 2: exponential gas = {}", day7::exponential_gas(&subs));
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day8" {
            let segments = day8::read_data();
            let now = Instant::now();
            println!("Part 1: number of known digits = {}", day8::count_known_values(&segments));
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            println!("Part 2: decode seven segments = {}", day8::decode_values(&segments));
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day9" {
            let grid = day9::read_grid();
            let now = Instant::now();
            println!("Part 1: low point risk score = {}", day9::count_low_points(&grid));
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            println!("Part 2: 3 largest basins = {}", day9::find_basins(&grid));
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day10" {
            let lines = day10::read_lines();
            let (illegal_score, incomplete_score) = day10::syntax_score(&lines);
            println!("Part 1: illegal line score = {}", illegal_score);
            println!("Part 2: completion line score = {}", incomplete_score);
        }
        if day == "day11" {
            let octopi = day11::read_octopi();
            println!("Part 1: bursts after 100 steps = {}", day11::flash_after_steps(&octopi, 100));
            println!("Part 2: step when all burst = {}", day11::find_all_flash(&octopi));
        }
        if day == "day12" {
            let graph = day12::read_paths();
            let now = Instant::now();
            println!("Part 1: all possible paths = {}", day12::count_total_paths(&graph));
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            println!("Part 2: all paths allowing double visit to small cave = {}", day12::count_paths_visit_twice(&graph));
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day13" {
            let (dots, instructions) = day13::read_data();
            let now = Instant::now();
            println!("Part 1: dots after one fold = {}", day13::dots_one_fold(&dots, &instructions[0]));
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            let after_folds = day13::fold_all(&dots, &instructions);
            println!("Day 2");
            for row in after_folds {
                for value in row.iter().map(|&val| if val {'#'} else {' '}) {
                    print!("{}", value);
                }
                println!("");
            }
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day14" {
            let (template, pair_insertion) = day14::read_polymer_data();
            let now = Instant::now();
            println!("Part 1: common polymers = {}", day14::common_polymers(&template, &pair_insertion, 10));
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            println!("Part 2: use pair based polymer count = {}", day14::polymers_as_pairs(&template, &pair_insertion, 40));
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day15" {
            let grid = day15::read_grid();
            let now = Instant::now();
            println!("Part 1: Lowest risk path = {}", day15::dijkstra(&grid));
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            let expanded = day15::expand_grid(&grid);
            println!("Part 2: Expanded risk path cost = {}", day15::dijkstra(&expanded));
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day16" {
            let packet = day16::read_packet();
            println!("Part 1: count version numbers = {}", packet.count_version());
            println!("Part 2: calculate packet value = {}", packet.calculate());
        }
        if day == "day17" {
            let target_area = day17::read_target_area();
            let now = Instant::now();
            println!("Part 1: highest possible height = {}", day17::highest_possible(&target_area));
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            println!("Part 2: total number of velocities = {}", day17::all_possible_velocities(&target_area));
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day18" {
            let numbers = day18::read_input();
            let now = Instant::now();
            let sum = day18::add_all(numbers);
            println!("Part 1: final sum magnitude = {}", sum.borrow().magnitude());
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            println!("Part 2: largest combo mangitude = {}", day18::largest_magnitude());
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day19" {
            let scanners = day19::read_input();
            let now = Instant::now();
            let (beacons, farthest) = day19::locate_beacons(&scanners);
            println!("Part 1: total number of beacons = {}", beacons);
            println!("Part 2: distance between two farthest scanners = {}", farthest);
            println!("Part 1&2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);

        }
        if day == "day20" {
            let (image, enhance) = day20::read_data();
            let now = Instant::now();
            println!("Part 1: Count after 2 enhance steps = {}", day20::count_after_steps(&image, &enhance, 2));
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            println!("Part 2: Count after 50 enhance steps = {}", day20::count_after_steps(&image, &enhance, 50));
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day21" {
            println!("Part 1: play a deterministic game = {}", day21::play_deterministic(6, 3));
            let now = Instant::now();
            println!("Part 2: winning player wins in {} universes", day21::dirac_dice(6, 3));
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day22" {
            let steps = day22::read_steps();
            let now = Instant::now();
            println!("Part 1: number of cubes on in -50,50 space = {}", day22::cubes_on_50(&steps));
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            println!("Part 2: total number of cubes on = {}", day22::all_cubes_on(&steps));
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day23" {
            let now = Instant::now();
            println!("Part 1: energy used = {}", day23::lowest_energy_solution(&day23::part_1_start()));
            println!("Part 1 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
            let now = Instant::now();
            println!("Part 2: energy used = {}", day23::lowest_energy_solution(&day23::part_2_start()));
            println!("Part 2 in {}ms", now.elapsed().as_nanos() as f64 / 1000_000.0);
        }
        if day == "day24" {
            let instructions = day24::read_instructions();
            let largest = "92928914999991";
            if day24::validate_modal_number(largest, &instructions) {
                println!("Part 1: Largest valid number = {}", largest);
            }
            let smallest = "91811211611981";
            if day24::validate_modal_number(smallest, &instructions) {
                println!("Part 1: Smallest valid number = {}", smallest);
            }
        }
        if day == "day25" {
            let grid = day25::read_grid();
            println!("Part 1: step when nothing moves = {}", day25::find_stable_step(&grid));
        }
    }
}
