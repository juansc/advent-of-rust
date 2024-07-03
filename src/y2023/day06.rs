use crate::solver::Solver;
use std::iter::zip;

pub struct Day6Solver {}

impl Solver for Day6Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let times = parse_race_info(
            lines[0]
                .splitn(2, "Time:")
                .collect::<Vec<_>>()
                .get(1)
                .unwrap(),
        );
        let distances_to_beat = parse_race_info(
            lines[1]
                .splitn(2, "Distance:")
                .collect::<Vec<_>>()
                .get(1)
                .unwrap(),
        );
        let race_info = zip(times, distances_to_beat)
            .map(|(t, d)| RaceInfo {
                race_duration_in_seconds: t,
                distance_to_beat: d,
            })
            .collect::<Vec<RaceInfo>>();

        let mut product = 1;
        for race in race_info {
            let strat = LinearSpeedStrategy {
                race_info: race,
            };
            product *= strat.winning_charge_durations().len();
        }
        product.to_string()
    }

    fn solve_part_2(&self, _: Vec<String>) -> String {
        // Not bothering to parse
        let race_info = RaceInfo {
            race_duration_in_seconds: 61677571,
            distance_to_beat: 430103613071150,
        };
        LinearSpeedStrategy{race_info}.winning_charge_durations().len().to_string()
    }
}

#[derive(Debug, Copy, Clone)]
struct RaceInfo {
    race_duration_in_seconds: usize,
    distance_to_beat: usize,
}

struct LinearSpeedStrategy {
    race_info: RaceInfo,
}

impl LinearSpeedStrategy {
    fn winning_charge_durations(&self) -> Vec<usize> {
        let mut out = vec![];
        // I know that this can be solved by using 
        // (T-h)*h - d > 0
        // where 
        // T = time for race
        // h = hold time
        // d = distance to beat
        // and this is a quadratic. You could optimize this in a few ways
        // 1. Find the zeros and count the distance between them
        // 2. Start from the ends (hold for 1 second, hold for T -1 second) and stop
        //    counting when you find the regions where you stop losing.
        // However, I thought we might have to do something interesting with the outcomes
        // so I just did this and it's fast enough.
        for i in 1..self.race_info.race_duration_in_seconds {
            let d = self.distance_for_hold_duration(i);
            if d > self.race_info.distance_to_beat as isize {
                out.push(i);
            }
        }
        out
    }

    fn distance_for_hold_duration(&self, hold: usize) -> isize {
        let dur = self.race_info.race_duration_in_seconds as isize;
        let hold = hold as isize;
        (dur - hold) * hold
    }
}

fn parse_race_info(line: &str) -> Vec<usize> {
    line.split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}
