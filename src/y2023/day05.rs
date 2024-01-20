use crate::solver;
use nom::character::complete::space1;
use nom::IResult;
use std::collections::HashMap;

pub struct Day5Solver {}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Range {
    // Both start and end are inclusive
    start: usize,
    end: usize,
    delta: isize,
}
impl Range {
    fn new(start: usize, end: usize, delta: isize) -> Self {
        Range { start, end, delta }
    }

    fn contains(&self, val: usize) -> bool {
        self.start <= val && val <= self.end
    }

    // split_on should only be called if the value is contained in the range
    // and does not match the endpoints.
    fn split_on(&self, val: usize) -> (Range, Range) {
        if val <= self.start || self.end <= val {
            panic!("cannot split_on if the value is not in the range and not the endpoints");
        }
        let first = Range::new(self.start, val - 1, self.delta);
        let second = Range::new(val, self.end, self.delta);
        (first, second)
    }
}

/*
struct NumberLine {
ranges: Vec<Range>,
}

impl NumberLine {
fn new() -> Self {
    NumberLine {
        ranges: vec![Range::new(0, usize::MAX, 0)],
    }
}
fn add_range(&mut self, new_range: Range) {
    // First, find all the ranges that currently overlap with the new range. That means that
    // you find the first range that contains the start, the range that contains the end, and
    // all the ones in between.
    let mut overlapping_ranges = vec![];
    let mut add_ranges = false;
    let mut first_idx = 0;
    let mut last_idx = 0;
    for (idx, r) in self.ranges.iter().enumerate() {
        // If we are in the middle of adding something just add it
        if add_ranges {
            overlapping_ranges.push(r);
            // Otherwise check if we should add for the first time
        } else if r.contains(new_range.start) {
            first_idx = idx;
            // Add this as the starting one.
            overlapping_ranges.push(r);
            // If this also contains the end then exit early
            add_ranges = true;
        }

        // If this also contains the end make a note and exit
        if r.contains(new_range.end) {
            last_idx = idx;
            break;
        }
    }
        // TODO: Only add ranges if they are the same length
    // +------+-----+-----+----------+
    //   +-------------------+
    // +-+----+-----+-----+--+-------+
    // We only have to potentially split the first and the last ranges. The ones in between
    // don't need to be split.
    if overlapping_ranges.len() == 1 {
        let mut range = *overlapping_ranges[0];
        let mut new_ranges = vec![];
        if range.start < new_range.start {
            let (first, mut second) = range.split_on(new_range.start);
            new_ranges.push(first);
            range = second;
        }
        if
        let (first, mut second) = range.split_on(new_range.start);
    }

    let mut new_ranges = vec![];
    for (idx, r) in overlapping_ranges.iter().enumerate() {
        if idx == 0 && r.start < new_range.start {
            let (first, mut second) = r.split_on(new_range.start);
            new_ranges.push(first);
            second.delta += new_range.delta;
            new_ranges.push(second);
            continue;
        }

        if idx == overlapping_ranges.len() - 1 && new_range.end < r.end {
            let (mut first, second) = r.split_on(new_range.end + 1);
            first.delta += new_range.delta;
            new_ranges.push(first);
            new_ranges.push(second);
            continue;
        }

        new_ranges.push(Range::new(r.start, r.end, r.delta + new_range.delta));
    }

    self.ranges.splice(first_idx..=last_idx, new_ranges);
}
}

#[cfg(test)]
mod number_line_tests {
use crate::y2023::day05::{NumberLine, Range};

#[test]
fn test_adding_one_range() {
    let mut number_line = NumberLine::new();
    number_line.add_range(Range::new(10, 20, 10));

    let ranges = number_line.ranges;
    assert_eq!(ranges[0], Range::new(0, 9, 0));
    assert_eq!(ranges[1], Range::new(10, 20, 10));
    assert_eq!(ranges[2], Range::new(21, usize::MAX, 0));
}

#[test]
fn test_adding_two_non_overlapping_range() {
    let mut number_line = NumberLine::new();
    number_line.add_range(Range::new(10, 20, 10));
    number_line.add_range(Range::new(30, 50, 5));

    let ranges = number_line.ranges;
    assert_eq!(ranges[0], Range::new(0, 9, 0));
    assert_eq!(ranges[1], Range::new(10, 20, 10));
    assert_eq!(ranges[2], Range::new(21, 29, 0));
    assert_eq!(ranges[3], Range::new(30, 50, 5));
    assert_eq!(ranges[4], Range::new(51, usize::MAX, 0));
}

#[test]
fn test_adding_two_overlapping_range() {
    let mut number_line = NumberLine::new();
    number_line.add_range(Range::new(10, 20, 10));
    number_line.add_range(Range::new(15, 25, 5));

    let ranges = number_line.ranges;
    assert_eq!(ranges[0], Range::new(0, 9, 0));
    assert_eq!(ranges[1], Range::new(10, 14, 10));
    assert_eq!(ranges[2], Range::new(15, 20, 15));
    assert_eq!(ranges[3], Range::new(21, 25, 5));
    assert_eq!(ranges[4], Range::new(26, usize::MAX, 0));
}
}

impl solver::Solver for Day5Solver {
fn solve_part_1(&self, lines: Vec<String>) -> String {
    let seeds = lines[0]
        .split("seeds: ")
        .collect::<Vec<_>>()
        .get(1)
        .unwrap()
        .split(" ")
        .collect::<Vec<_>>();
    let lines = &lines[2..];
    let mapper_info: Vec<&[String]> = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
    let mut mappers = vec![];
    for mapper in mapper_info.iter() {
        let mut name = "";
        let mut maps = vec![];
        for (idx, line) in mapper.iter().enumerate() {
            if idx == 0 {
                name = line;
                continue;
            }
            let (_, map) = parse_range_mapper(line).unwrap();
            maps.push(map);
        }
        mappers.push(CombinedRangeMapper::new(name.to_string(), maps));
    }
    println!("Seeds: {:?}", seeds);
    println!("{:?}", mappers);

    let mut min_seed_value = usize::MAX;

    for seed in seeds {
        let mut val = seed.parse::<usize>().unwrap();
        println!("Will evalute seed {}", seed);
        for map in mappers.iter_mut() {
            println!("{:?}", map);
            let new_val = map.map(val);
            println!("{} maps to {} in mapper {}", val, new_val, map.name.clone());
            val = new_val;
        }
        println!("Seed {} maps to {}", seed, val);
        if val < min_seed_value {
            min_seed_value = val;
        }
    }
    min_seed_value.to_string()
}

fn solve_part_2(&self, lines: Vec<String>) -> String {
    let seeds = lines[0]
        .split("seeds: ")
        .collect::<Vec<_>>()
        .get(1)
        .unwrap()
        .split(" ")
        .collect::<Vec<_>>();
    let lines = &lines[2..];
    let mapper_info: Vec<&[String]> = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
    let mut mappers = vec![];
    for mapper in mapper_info.iter() {
        let mut name = "";
        let mut maps = vec![];
        for (idx, line) in mapper.iter().enumerate() {
            if idx == 0 {
                name = line;
                continue;
            }
            let (_, map) = parse_range_mapper(line).unwrap();
            maps.push(map);
        }
        mappers.push(CombinedRangeMapper::new(name.to_string(), maps));
    }
    println!("Seeds: {:?}", seeds);
    println!("{:?}", mappers);

    let mut min_seed_value = usize::MAX;

    for seed_range in seeds.chunks(2) {
        let start = seed_range[0].parse::<usize>().unwrap();
        let end = start + seed_range[1].parse::<usize>().unwrap();
        for seed in start..end {
            let mut val = seed;
            // println!("Will evalute seed {}", seed);
            for map in mappers.iter_mut() {
                let new_val = map.map(val);
                //  println!("{} maps to {} in mapper {}", val, new_val, map.name.clone());
                val = new_val;
            }
            //println!("Seed {} maps to {}", seed, val);
            if val < min_seed_value {
                min_seed_value = val;
            }
        }
    }
    min_seed_value.to_string()
}
}

fn parse_range_mapper(line: &str) -> IResult<&str, RangeMapper> {
let (line, dest_start) = parse_number(line)?;
let (line, _) = space1(line)?;
let (line, source_start) = parse_number(line)?;
let (line, _) = space1(line)?;
let (line, length) = parse_number(line)?;
Ok((
    line,
    RangeMapper {
        source_start,
        dest_start,
        length,
    },
))
}

#[derive(Debug)]
struct CombinedRangeMapper {
name: String,
mappers: Vec<RangeMapper>,

cache: HashMap<usize, usize>,
}

impl CombinedRangeMapper {
// map iterates over all mappers. If any of them contain the value they will map it, otherwise
// it will return the original value.
fn map(&mut self, source_val: usize) -> usize {
    /*
    if let Some(cached_val) = self.cache.get(&source_val) {
        return *cached_val;
    }
    jjjjjjjjj
     */
    if self.cache.contains_key(&source_val) {
        return self.cache.get(&source_val).unwrap().clone();
    }
    for map in self.mappers.iter() {
        // At this point we have passed any mapper that would've have picked it up
        if source_val < map.source_start {
            self.cache.insert(source_val, source_val);
            return source_val;
        }
        // The maps are sorted by order. This means that we will not find any more mappers.
        if source_val >= map.source_start + map.length {
            continue;
        }
        //            if map.in_range(source_val) {
        let out = map.map(source_val);
        self.cache.insert(source_val, out);
        return out;
        //
        // }
    }
    source_val
}

fn new(name: String, mappers: Vec<RangeMapper>) -> Self {
    // Sort mappers by their source start, where the smaller source start comes first
    let mut mappers = mappers;
    mappers.sort_by(|a, b| a.source_start.cmp(&b.source_start));
    CombinedRangeMapper {
        name,
        mappers,
        cache: HashMap::new(),
    }
}
}

#[derive(Debug)]
struct RangeMapper {
source_start: usize,
dest_start: usize,
length: usize,
}

impl RangeMapper {
fn map(&self, source_val: usize) -> usize {
    if source_val < self.source_start || source_val >= self.source_start + self.length {
        return source_val;
    }
    let offset = source_val - self.source_start;
    self.dest_start + offset
}

fn in_range(&self, source_val: usize) -> bool {
    source_val >= self.source_start && source_val < self.source_start + self.length
}
}

#[cfg(test)]
mod tests {
use crate::solver::Solver;
use crate::utils::lines::lines_from_file;

use super::*;

#[test]
fn test_part_1_unit() {
    let solver = Day5Solver {};
    let lines = [
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(solver.solve_part_1(lines), "35");
}

#[test]
fn test_part_1() {
    let solver = Day5Solver {};
    let lines = lines_from_file("./inputs/day05.txt");
    assert_eq!(solver.solve_part_1(lines), "551761867");
}

#[test]
fn test_part_2_unit() {
    let solver = Day5Solver {};
    let lines = [
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(solver.solve_part_2(lines), "46");
}

#[test]
fn test_part_2() {
    let solver = Day5Solver {};
    let lines = lines_from_file("./inputs/day05.txt");
    assert_eq!(solver.solve_part_2(lines), "551761867");
}
}


 */