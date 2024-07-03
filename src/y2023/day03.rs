use std::collections::HashMap;
use crate::solver;

pub struct Day3Solver {}

struct Schema {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
    symbol_locations: HashMap<isize, Vec<Symbol>>,
    part_locations: HashMap<isize, Vec<Part>>,
}

impl Schema {
    fn parts_adjacent_to_point(&self, location: &Location) -> Vec<Part> {
        let row = location.y;
        let part_candidates: Vec<Part> = vec![
            self.part_locations.get(&(row - 1)).cloned().unwrap_or(vec![]),
            self.part_locations.get(&row).cloned().unwrap_or(vec![]),
            self.part_locations.get(&(row + 1)).cloned().unwrap_or(vec![]),
        ].into_iter().flatten().collect();

        // The most number of parts that can be adjacent to a point is 8.
        let mut adjacent_points = Vec::with_capacity(8);

        for part in part_candidates.iter() {
            if part.is_adjacent_to(location) {
                adjacent_points.push(*part);
            }
        }
        adjacent_points
    }
}

fn schema_from_lines(lines: Vec<String>) -> Schema {
    let mut parts = vec![];
    let mut symbols = vec![];
    let mut part_locations: HashMap<isize, Vec<Part>> = HashMap::new();
    let mut symbol_locations: HashMap<isize, Vec<Symbol>> = HashMap::new();
    for (current_y, line) in lines.iter().enumerate() {
        let current_y = current_y as isize;
        let mut on_part = false;
        let mut starting_idx = 0isize;
        for (current_x, c) in line.chars().enumerate() {
            let current_x = current_x as isize;
            let is_number = c.is_numeric();
            if on_part && is_number {
                continue;
            }
            if on_part && !is_number {
                let part = Part::new(line[starting_idx as usize..current_x as usize].parse().unwrap(), Location { x: starting_idx, y: current_y }, (current_x - starting_idx) as usize);
                parts.push(part);
                part_locations.entry(current_y).or_default().push(part);
                on_part = false;
            }
            if !is_number {
                if c == '.' {
                    continue;
                }
                let symbol = Symbol {
                    symbol: c,
                    location: Location { x: current_x, y: current_y },
                };
                symbol_locations.entry(current_y).or_default().push(symbol);
                symbols.push(symbol);
                continue;
            }

            if !on_part && is_number {
                on_part = true;
                starting_idx = current_x;
            }
        }
        if on_part {
            let part = Part::new(line[starting_idx as usize..].parse().unwrap(), Location { x: starting_idx, y: current_y }, (line.len() as isize - starting_idx) as usize);
            part_locations.entry(current_y).or_default().push(part);
            parts.push(part);
        }
    }
    Schema {
        parts,
        symbols,
        symbol_locations,
        part_locations,
    }
}

impl solver::Solver for Day3Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        // We capture the symbols with a HashMap<row, Vec<symbol>>. The symbols are ordered left to right.
        // As soon as we find one symbol that is adjacent to a part or a symbol that is too far to the
        // right, we can stop looking for that symbol.
        // We create a vector of all the parts.
        let schema = schema_from_lines(lines);
        let mut sum_of_actual_parts = 0;
        for part in schema.parts {
            let part_row = part.location.y;
            let has_adjacent_symbols = (part_row - 1..=part_row + 1)
                .flat_map(|row| schema.symbol_locations.get(&row).map_or(Vec::new(), |symbols| symbols.clone()))
                .any(|symbol| part.is_adjacent_to(&symbol.location));

            if has_adjacent_symbols {
                sum_of_actual_parts += part.id;
            }
        }
        sum_of_actual_parts.to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        let schema = schema_from_lines(lines);
        let mut sum_gear_ratios = 0usize;
        for symbol in schema.symbols.iter() {
            // Skip if it's not a gear
            if symbol.symbol != '*' {
                continue;
            }

            let adjacent_parts = schema.parts_adjacent_to_point(&symbol.location);
            // Too many adjacent parts to be a gear ratio
            if adjacent_parts.len() != 2 {
                continue;
            }
            sum_gear_ratios += adjacent_parts[0].id * adjacent_parts[1].id;
        }
        sum_gear_ratios.to_string()
    }
}

#[derive(Debug, Copy, Clone)]
struct Part {
    id: usize,
    location: Location,
    width: usize,
}

impl Part {
    fn new(id: usize, location: Location, width: usize) -> Part {
        Part {
            id,
            location,
            width,
        }
    }

    fn is_adjacent_to(&self, location: &Location) -> bool {
        // The part can be represented as a 1 x width segment. Create a bounding box of
        // 3 x width + 2, and check if the symbol is within that box. In the diagram below
        // the segment is made with • and the bounding box is made with +.
        // +++++++
        // +•••••+
        // +++++++
        let min_x = self.location.x - 1;
        // Looks stupid, but I want to capture that the end of the segment is
        // self.location.0 + self.width - 1, and then we move to the right by one.
        let max_x = (self.location.x + self.width as isize - 1) + 1;
        let min_y = self.location.y - 1;
        let max_y = self.location.y + 1;
        //println!("At part {:?}", self);
        //println!("Does symbol {:?} fit in bounding box ({}, {}, {}, {})?", symbol, min_x, max_x, min_y, max_y);
        min_x <= location.x && location.x <= max_x && min_y <= location.y && location.y <= max_y
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Symbol {
    symbol: char,
    location: Location,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Location {
    x: isize,
    y: isize,
}

#[cfg(test)]
mod tests {
    use crate::y2023::day03::Day3Solver;
    use crate::utils::lines::lines_from_file;
    use crate::solver::Solver;

    #[test]
    fn test_part_1_unit() {
        let solver = Day3Solver {};
        let lines = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        assert_eq!(solver.solve_part_1(lines), "4361");
    }

    #[test]
    fn test_part_2_unit() {
        let solver = Day3Solver {};
        let lines = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        assert_eq!(solver.solve_part_2(lines), "467835");
    }

    #[test]
    fn test_part_1() {
        let solver = Day3Solver {};
        let lines = lines_from_file("./inputs/day03.txt");
        assert_eq!(solver.solve_part_1(lines), "550064");
    }

    #[test]
    fn test_part_2() {
        let solver = Day3Solver {};
        let lines = lines_from_file("./inputs/day03.txt");
        assert_eq!(solver.solve_part_2(lines), "85010461");
    }
}