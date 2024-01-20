use std::env;

mod utils;
mod y2023;
mod solver;

fn main() {
    // Read from stdin
    let args: Vec<String> = env::args().collect();

    // Parse first argument as a uint8
    let year: usize = args[1].parse().unwrap();
    let day: u8 = args[2].parse().unwrap();

    let solver = get_solver(year, day).unwrap();
    // read file contents as an array of lines without using include_str
//    let input = format!("./inputs/unit_test/day{:02}.txt", day);
    let input = format!("./inputs/{:04}-day{:02}.txt", year, day);
    // rust, read a file as a vector of strings
    let lines = utils::lines::lines_from_file(input);
    println!("Part 1: {}", solver.solve_part_1(lines.clone()));
    println!("Part 2: {}", solver.solve_part_2(lines));
}

fn get_solver(year: usize, day: u8) -> Option<Box<dyn solver::Solver>> {
    match year {
        2023 => {
            match day {
                1 => Some(Box::new(y2023::day01::Day1Solver {})),
                2 => Some(Box::new(y2023::day02::Day2Solver {})),
                3 => Some(Box::new(y2023::day03::Day3Solver {})),
                4 => Some(Box::new(y2023::day04::Day4Solver {})),
                _ => None,
            }
        }
        _ => None,
    }
}