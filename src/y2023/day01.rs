use crate::solver;

pub struct Day1Solver {}

impl solver::Solver for Day1Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut digits: Vec<u32> = vec![];
        for line in lines {
            // If it's a digit push it, otherwise move on
            let line_digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            let l = line_digits.len();
            digits.push(10 * line_digits[0] + line_digits[l - 1]);
        }
        digits.iter().sum::<u32>().to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        let mut digits: Vec<u64> = vec![];
        for line in lines {
            // If it's a digit push it, otherwise move on
            let mut line_digits = vec![];
            let mut parser = NumberParser::new(&line);
            while parser.has_next() {
                if let Some(d) = parser.parse_at_current() {
                    line_digits.push(d as u64);
                }
            }
            let l = line_digits.len();
            digits.push(10 * line_digits[0] + line_digits[l - 1]);
        }
        digits.iter().sum::<u64>().to_string()
    }
}

struct NumberParser<'a> {
    // input has the lifetime of the NumberParser
    input: &'a str,
    index: usize,
}

impl<'a> NumberParser<'a> {
    fn new(input: &str) -> NumberParser {
        NumberParser {
            input,
            index: 0,
        }
    }

    fn has_next(&self) -> bool {
        self.index < self.input.len()
    }

    fn parse_at_current(&mut self) -> Option<u8> {
        if !self.has_next() {
            return None;
        }
        let c = self.input.as_bytes()[self.index];
        if c.is_ascii_digit() {
            self.index += 1;
            return Some(c - b'0');
        }
        let remaining_length = self.input.len() - self.index;
        let candidates = std::collections::HashSet::from(
            [
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ],
        );
        for candidate in candidates {
            if candidate.0.len() <= remaining_length && &self.input[self.index..self.index + candidate.0.len()] == candidate.0 {
                // Note that twone is two and then one, so we can't skip the length of the word
                self.index += 1;
                return Some(candidate.1);
            }
        }
        self.index += 1;
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::Solver;
    use crate::utils::lines::lines_from_file;

    #[test]
    fn test_part_1_unit() {
        let solver = Day1Solver {};
        let lines = vec!["1234".to_string()];
        assert_eq!(solver.solve_part_1(lines), "14");
        let lines = vec!["1xx4".to_string()];
        assert_eq!(solver.solve_part_1(lines), "14");
        let lines = vec!["1xxx".to_string()];
        assert_eq!(solver.solve_part_1(lines), "11", "single number");
        let lines = vec!["100000005x0".to_string()];
        assert_eq!(solver.solve_part_1(lines), "10");
    }

    #[test]
    fn test_part_1() {
        let solver = Day1Solver {};
        let lines = lines_from_file("./inputs/day01.txt");
        assert_eq!(solver.solve_part_1(lines), "55002");
    }

    #[test]
    fn test_part_2_unit() {
        let solver = Day1Solver {};
        let lines = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];
        assert_eq!(solver.solve_part_2(lines), "281");
    }

    #[test]
    fn test_part_2_single_number() {
        let solver = Day1Solver {};
        let lines = vec![
            "v4".to_string(),
        ];
        assert_eq!(solver.solve_part_2(lines), "44");
    }

    #[test]
    fn test_part_2() {
        let solver = Day1Solver {};
        let lines = lines_from_file("./inputs/day01.txt");
        assert_eq!(solver.solve_part_2(lines), "55093");
    }

    #[test]
    fn test_number_parser() {
        let mut parser = NumberParser::new("1234");
        assert_eq!(parser.parse_at_current(), Some(1));
        assert_eq!(parser.parse_at_current(), Some(2));
        assert_eq!(parser.parse_at_current(), Some(3));
        assert_eq!(parser.parse_at_current(), Some(4));
        assert_eq!(parser.parse_at_current(), None);
    }

    #[test]
    fn test_number_parser_some_letters() {
        let mut parser = NumberParser::new("1xx4");
        assert_eq!(parser.parse_at_current(), Some(1));
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), Some(4));
        assert_eq!(parser.parse_at_current(), None);
    }

    #[test]
    fn test_with_named_numbers() {
        let mut parser = NumberParser::new("xonextwothree4xfiv6e");
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), Some(1));
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), Some(2));
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), Some(3));
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), Some(4));
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), None);
        assert_eq!(parser.parse_at_current(), Some(6));
        assert_eq!(parser.parse_at_current(), None);
    }

    #[test]
    fn test_with_more_lines() {
        let mut parser = NumberParser::new("tsgbzmgbonethreedrqzbhxjkvcnm3");
        let mut actual_digits = vec![];
        while parser.has_next() {
            let val = parser.parse_at_current();
            if val.is_some() {
                actual_digits.push(val.unwrap());
            }
        }
        assert_eq!(actual_digits, vec![1, 3, 3]);
    }
}
