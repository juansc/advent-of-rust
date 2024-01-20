use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{map, map_res, value};
use nom::IResult;
use nom::multi::separated_list1;
use crate::solver;


pub struct Day2Solver {}

impl solver::Solver for Day2Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let max_reveal = RevealedDice {
            red: 12,
            blue: 14,
            green: 13,
        };
        let mut sum_of_valid_games = 0;
        for line in lines {
            let (_, game) = parse_game_line(&line).unwrap();
            if game.reveals_less_than(&max_reveal) {
                sum_of_valid_games += game.id;
            }
        }
        sum_of_valid_games.to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        let mut sum_of_powers = 0usize;
        for line in lines {
            let (_, game) = parse_game_line(&line).unwrap();
            let min_dice = game.min_dice_required();
            sum_of_powers += min_dice.red as usize * min_dice.blue as usize * min_dice.green as usize;
        }
        sum_of_powers.to_string()
    }
}

#[derive(Debug, Copy, Clone)]
enum DieColor {
    Red,
    Blue,
    Green,
}

#[derive(Debug, Clone)]
struct GameLine {
    id: usize,
    game_reveals: Vec<RevealedDice>,
}

impl GameLine {
    fn reveals_less_than(&self, maximum: &RevealedDice) -> bool {
        for reveal in &self.game_reveals {
            if reveal.red > maximum.red || reveal.blue > maximum.blue || reveal.green > maximum.green {
                return false;
            }
        }
        true
    }

    fn min_dice_required(&self) -> RevealedDice {
        let mut min_dice = RevealedDice {
            red: 0,
            blue: 0,
            green: 0,
        };
        for reveal in &self.game_reveals {
            if reveal.red > min_dice.red {
                min_dice.red = reveal.red;
            }
            if reveal.blue > min_dice.blue {
                min_dice.blue = reveal.blue;
            }
            if reveal.green > min_dice.green {
                min_dice.green = reveal.green;
            }
        }
        min_dice
    }
}

#[derive(Debug, Copy, Clone)]
struct RevealedDice {
    red: u8,
    blue: u8,
    green: u8,
}


fn parse_game_line(line: &str) -> IResult<&str, GameLine> {
    let (line, _) = tag("Game ")(line)?;
    let (line, id) = parse_number(line)?;
    let (line, _) = tag(": ")(line)?;
    let (line, game_reveals) = separated_list1(
        tag("; "),
        parse_revealed_dice,
    )(line)?;
    Ok((line, GameLine {
        id,
        game_reveals,
    }))
}

fn parse_revealed_dice(line: &str) -> IResult<&str, RevealedDice> {
    let (line, die_colors) = separated_list1(
        tag(", "),
        parse_die_colors,
    )(line)?;
    let mut revealed = RevealedDice {
        red: 0,
        blue: 0,
        green: 0,
    };
    for (num, color) in die_colors {
        match color {
            DieColor::Red => revealed.red = num,
            DieColor::Blue => revealed.blue = num,
            DieColor::Green => revealed.green = num,
        }
    }
    Ok((line, revealed))
}

fn parse_die_colors(line: &str) -> IResult<&str, (u8, DieColor)> {
    map(
        nom::sequence::tuple(
            (
                parse_number,
                tag(" "),
                parse_color,
            )
        )
        , |(usize, _, color)| {
            (usize as u8, color)
        })(line)
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    // take_while1 grabs as many numbers as it can
    // map_res takes the result of take_while1 and tries to parse it as a usize
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

fn parse_color(i: &str) -> IResult<&str, DieColor> {
    nom::branch::alt(
        (
            value(DieColor::Red, tag("red")),
            value(DieColor::Blue, tag("blue")),
            value(DieColor::Green, tag("green")),
        )
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::Solver;
    use crate::utils::lines::lines_from_file;
    use crate::y2023::day01::Day1Solver;

    #[test]
    fn test_part_1_unit_test() {
        let lines = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green   ".to_string(),
        ];
        let solver = Day2Solver {};
        assert_eq!(solver.solve_part_1(lines), "8");
    }

    #[test]
    fn test_part_1() {
        let solver = Day1Solver {};
        let lines = lines_from_file("./inputs/day02.txt");
        assert_eq!(solver.solve_part_1(lines), "2317");
    }

    #[test]
    fn test_part_2_unit_test() {
        let lines = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green   ".to_string(),
        ];
        let solver = Day2Solver {};
        assert_eq!(solver.solve_part_2(lines), "2286");
    }
}