use nom::character::complete::space1;
use nom::IResult;
use crate::solver::Solver;
use crate::utils::parsers::parse_number;

#[derive(Debug)]
struct MapRangeCombiner {
    name: String,
    mappers: Vec<MapRangeLayer>,
}

impl MapRangeCombiner {
    fn evaluate_range(&self, range: &Range, map_index: usize) -> Vec<Range> {
        // If there is nothing left to map to return yourself
        if map_index >= self.mappers.len() {
            return vec![*range];
        }
        let mut out = vec![];
        if let Some(current_mapper) = self.mappers.get(map_index) {
            // For the current mapper evaluate the range and return all the new values we found.
            for new_mapped_range in current_mapper.evaluate_range(range).iter() {
                out.extend(self.evaluate_range(new_mapped_range, map_index + 1));
            }
        }
        out
    }
}

pub struct Day5Solver {}

fn parse_range_mapper(line: &str) -> IResult<&str, MapRange> {
    let (line, dest_start) = parse_number(line)?;
    let (line, _) = space1(line)?;
    let (line, source_start) = parse_number(line)?;
    let (line, _) = space1(line)?;
    let (line, length) = parse_number(line)?;
    Ok((
        line,
        MapRange::new(
            source_start,
            source_start + length - 1,
            dest_start as isize - source_start as isize,
        ),
    ))
}

impl Solver for Day5Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let seeds = lines[0]
            .split("seeds: ")
            .collect::<Vec<_>>()
            .get(1)
            .unwrap()
            .split(' ')
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
            // We've created all the number ranges. Collect them into a MapRangeLayer.
            mappers.push(MapRangeLayer::from_ranges(name.into(), maps));
        }
        let evaluator = MapRangeCombiner { name: "winner".to_string(), mappers };

        let mut min_seed_value = usize::MAX;

        for seed in seeds {
            // Create ranges that have a single element and find the min out of all of them.
            let val = seed.parse::<usize>().unwrap();
            let seed_ranges = vec![Range::new(val, val)];
            for seed_range in seed_ranges {
                let values = evaluator.evaluate_range(&seed_range, 0);
                for val in values {
                    if val.start < min_seed_value {
                        min_seed_value = val.start;
                    }
                }
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
            .split(' ')
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
            // We've created all the number ranges. Collect them into a MapRangeLayer.
            mappers.push(MapRangeLayer::from_ranges(name.into(), maps));
        }
        let evaluator = MapRangeCombiner { name: "winner".to_string(),  mappers,  };

        let seed_ranges = seeds.chunks(2).map(|chunk| {
            let seed_start = chunk[0].parse::<usize>().unwrap();
            let seed_range = chunk[1].parse::<usize>().unwrap();
            Range::new(seed_start, seed_start + seed_range - 1)
        }).collect::<Vec<Range>>();

        find_min_location_for_seed_range(seed_ranges, &evaluator).to_string()
    }
}

fn find_min_location_for_seed_range(seed_ranges: Vec<Range>, evaluator: &MapRangeCombiner) -> usize {
    let mut min_seed_value = usize::MAX;
    for seed_range in seed_ranges {
        for val in evaluator.evaluate_range(&seed_range, 0) {
            if val.start < min_seed_value {
                min_seed_value = val.start;
            }
        }
    }
    min_seed_value
}

#[cfg(test)]
mod tests_mind {
    use crate::y2023::day05::*;

    #[test]
    fn test_seed_ranges_min_location() {
        let seed_ranges = vec![Range::new(1, 10)];
        let map_range_combiner = MapRangeCombiner {
            mappers: vec![
                MapRangeLayer::from_ranges(
                    "".to_string(),
                    vec![
                        MapRange::new(1, 5, 5),
                        MapRange::new(6, 10, -4),
                    ],
                ),
                MapRangeLayer::from_ranges(
                    "".to_string(),
                    vec![
                        MapRange::new(1, 3, 7),
                        MapRange::new(4, 6, -3),
                        MapRange::new(7, 10, -3),
                    ],
                ),
            ],
            name: "winner".to_string(),
        };
        assert_eq!(find_min_location_for_seed_range(seed_ranges, &map_range_combiner), 1);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Range {
    // Both start and end are inclusive
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start) || self.contains(other.end) || other.contains(self.start) || other.contains(self.end)
    }

    fn contains(&self, val: usize) -> bool {
        self.start <= val && val <= self.end
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        if !self.overlaps(other) {
            return None;
        }

        // Since they overlap their overlap must be the max of the starts and the min of the ends
        Some(Self::new(std::cmp::max(self.start, other.start), std::cmp::min(self.end, other.end)))
    }

    fn shift(&self, delta: isize) -> Range {
        Range::new((self.start as isize + delta) as usize, (self.end as isize + delta) as usize)
    }

    // split_on should only be called if the value is contained in the range
    // and does not match the endpoints.
    fn split_on(&self, val: usize) -> (Self, Self) {
        if val <= self.start || self.end <= val {
            panic!("cannot split_on if the value is not in the range and not the endpoints");
        }
        (Self::new(self.start, val - 1), Self::new(val, self.end))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct MapRange {
    range: Range,
    delta: isize,
}

impl MapRange {
    fn new(start: usize, end: usize, delta: isize) -> Self {
        MapRange { range: Range::new(start, end), delta }
    }

    fn evaluate_range(&self, range: &Range) -> (Vec<Range>, Option<Range>) {
        let mut out = vec![];
        if let Some(intersection) = self.range.intersect(range) {
            // The part of the range that is to the left of the intersection maps to itself
            if range.start != intersection.start {
                out.push(Range::new(range.start, intersection.start - 1));
            }
            // Take the intersection and shift by delta
            out.push(intersection.shift(self.delta));
            // Return what it mapped to
            let remaining = (intersection.end == range.end).then_some(Range::new(intersection.end + 1, range.end));
            return (out, remaining);
        }
        (out, Some(*range))
    }
}

#[derive(Debug)]
struct MapRangeLayer {
    name: String,
    // A MapRangeLayer is a collection of non-overlapping MapRangeLayers. This can be interpreted
    // as a piece-wise function F(x) = {x + A if x in [a, b], x + B if x in [c, d], ...}
    ranges: Vec<MapRange>,
}

impl MapRangeLayer {
    /// mapped_ranges returns the ranges that the given range maps to. Note that this may be
    /// one or more, since the MapRangeLayer has piece-wise functions and the initial range may be
    /// split.
    fn evaluate_range(&self, range: &Range) -> Vec<Range> {
        let mut out = vec![];
        let mut remaining = *range;
        for r in self.ranges.iter() {
            let (intersection, not_intersected) = r.evaluate_range(&remaining);
            // We are no longer in the range that has intersected. Exit early
            out.extend(intersection);
            if not_intersected.is_none() {
                break;
            }
            remaining = not_intersected.unwrap();
        }
        out
    }
    fn from_ranges(name: String, ranges: Vec<MapRange>) -> Self {
        let mut new_ranges = vec![];
        // Sort ranges by start asc
        let mut ranges = ranges;
        ranges.sort_by(|a, b| a.range.start.cmp(&b.range.start));

        let mut start_index = 0;
        for range in ranges {
            if start_index < range.range.start {
                new_ranges.push(MapRange::new(start_index, range.range.start - 1, 0));
            }
            new_ranges.push(range);
            start_index = range.range.end + 1;
        }
        // Add the last range that goes to Inf. This is allowed because we
        // don't expect the caller to set it to inf.
        new_ranges.push(MapRange::new(start_index, usize::MAX, 0));
        Self { name, ranges: new_ranges }
    }
}

