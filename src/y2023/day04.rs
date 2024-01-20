use std::collections::{HashSet};
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use crate::solver;

pub struct Day4Solver {}

impl solver::Solver for Day4Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut points_won = 0;
        for line in lines {
            let (_, card) = parse_card(&line).unwrap();
            let points = card.points();
            points_won += points;
        }
        points_won.to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        // Each index represents the number of cards we have so far for each
        let mut cards_to_process = vec![1; lines.len()];
        for (idx, line) in lines.iter().enumerate() {
            // By the time we've gotten to this card we have a certain number of cards we need to resolve
            let num_copies_of_card = cards_to_process[idx];
            let (_, card) = parse_card(&line).unwrap();
            let points = card.num_matches();
            for i in 0..points {
                // We get a copy for each match, but we have to multiply it by the number of copies
                // we had by the time we got to this card
                cards_to_process[card.number + i] += 1 * num_copies_of_card;
            }
        }
        cards_to_process.iter().sum::<isize>().to_string()
    }
}

// A Card has a number, a HashSet of winning numbers and a
// HashSet of numbers that are present
struct Card {
    number: usize,
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl Card {
    // points are calculated by counting the number of the intersection between
    // winning_numbers and numbers and raising two to that power
    fn points(&self) -> usize {
        let count = self.num_matches();
        if count == 0 {
            return 0;
        }
        2usize.pow((count - 1) as u32)
    }

    fn num_matches(&self) -> usize {
        let intersection = self.winning_numbers.intersection(&self.numbers);
        intersection.count()
    }
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    // take_while1 grabs as many numbers as it can
    // map_res takes the result of take_while1 and tries to parse it as a usize
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

fn parse_card(line: &str) -> IResult<&str, Card> {
    let (line, _) = tag("Card")(line)?;
    let (line, _) = space1(line)?;
    let (line, number) = parse_number(line)?;
    let (line, _) = tag(":")(line)?;
    let (line, _) = space1(line)?;
    let (line, winning_numbers) = separated_list1(space1, parse_number)(line)?;
    let (line, _) = tag(" |")(line)?;
    let (line, _) = space1(line)?;
    let (line, numbers) = separated_list1(space1, parse_number)(line)?;
    Ok((line, Card {
        number,
        winning_numbers: HashSet::from_iter(winning_numbers),
        numbers: HashSet::from_iter(numbers),
    }))
}

#[cfg(test)]
mod tests {
    use crate::solver::Solver;
    use crate::utils::lines::lines_from_file;
    use crate::y2023::day04::Day4Solver;

    #[test]
    fn test_part_1_unit_test() {
        let solver = Day4Solver {};
        let lines = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ].iter().map(|s| s.to_string()).collect();
        let result = solver.solve_part_1(lines);
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part_1() {
        let solver = Day4Solver {};
        let lines = lines_from_file("inputs/day04.txt");
        let result = solver.solve_part_1(lines);
        assert_eq!(result, "18519");
    }

    #[test]
    fn test_part_2_unit_test() {
        let solver = Day4Solver {};
        let lines = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ].iter().map(|s| s.to_string()).collect();
        let result = solver.solve_part_2(lines);
        assert_eq!(result, "30");
    }

    #[test]
    fn test_part_2() {
        let solver = Day4Solver {};
        let lines = lines_from_file("inputs/day04.txt");
        let result = solver.solve_part_2(lines);
        assert_eq!(result, "11787590");
    }
}
