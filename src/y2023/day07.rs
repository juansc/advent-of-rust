use crate::solver::Solver;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;
use crate::y2023::day07::CardRank::Joker;
use crate::y2023::day07::RuleSet::{Simple, WithJokers};

pub struct Day7Solver {}

impl Solver for Day7Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut hands = vec![];
        for line in lines {
            let parts = line.split(' ').collect::<Vec<_>>();
            let poker_hand = CamelPokerHand::new_with_rules(parts[0].to_string(), Simple);
            let bid = parts[1].parse::<usize>().unwrap();
            hands.push(CamelPokerRound {
                hand: poker_hand,
                bid,
            });
        }

        hands.sort_by(|a, b| a.hand.cmp(&b.hand));
        let mut out = 0;
        for (mult, hand) in hands.iter().enumerate() {
            out += (mult + 1) * hand.bid;
        }
        out.to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        let mut hands = vec![];
        for line in lines {
            let parts = line.split(' ').collect::<Vec<_>>();
            let poker_hand = CamelPokerHand::new_with_rules(parts[0].to_string(), WithJokers);
            let bid = parts[1].parse::<usize>().unwrap();
            hands.push(CamelPokerRound {
                hand: poker_hand,
                bid,
            });
        }

        hands.sort_by(|a, b| a.hand.cmp(&b.hand));
        let mut out = 0;
        for (mult, hand) in hands.iter().enumerate() {
            out += (mult + 1) * hand.bid;
        }
        out.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::Solver;
    use crate::utils::lines::lines_from_file;
    use crate::y2023::day07::{CamelPokerHand, Day7Solver};

    #[test]
    fn compare_hands() {
        // Equal hands
        let hand_1 = CamelPokerHand::new("AAAKK".to_string());
        let hand_2 = CamelPokerHand::new("AAAKK".to_string());
        assert_eq!(hand_1 == hand_2, true);

        // Five of a kind is greater than full house
        let hand_1 = CamelPokerHand::new("AAAAA".to_string());
        let hand_2 = CamelPokerHand::new("AAAKK".to_string());
        assert_eq!(hand_1 > hand_2, true);

        // Cards of equal level where the first card breaks the tie in favor of the second hand
        let hand_1 = CamelPokerHand::new("KKAAA".to_string());
        let hand_2 = CamelPokerHand::new("AAKKK".to_string());
        assert_eq!(hand_1 < hand_2, true);
    }

    #[test]
    fn test_sorting() {
        let mut cards = vec![
            CamelPokerHand::new("32T3K".to_string()),
            CamelPokerHand::new("T55J5".to_string()),
            CamelPokerHand::new("KK677".to_string()),
            CamelPokerHand::new("KTJJT".to_string()),
            CamelPokerHand::new("QQQJA".to_string()),
        ];

        cards.sort();
        cards.reverse();

        assert_eq!(
            cards,
            vec![
                CamelPokerHand::new("QQQJA".to_string()),
                CamelPokerHand::new("T55J5".to_string()),
                CamelPokerHand::new("KK677".to_string()),
                CamelPokerHand::new("KTJJT".to_string()),
                CamelPokerHand::new("32T3K".to_string()),
            ]
        );
    }

    #[test]
    fn test_part_1_partial() {
        let lines = vec![
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483 ".to_string(),
        ];
        let solver = Day7Solver {};
        assert_eq!(solver.solve_part_1(lines), "6440");
    }

    #[test]
    fn test_part_2_partial() {
        let lines = vec![
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483 ".to_string(),
        ];
        let solver = Day7Solver {};
        assert_eq!(solver.solve_part_2(lines), "5905");
    }

    #[test]
    fn test_part_1() {
        let solver = Day7Solver {};
        let lines = lines_from_file("inputs/2023-day07.txt");
        let result = solver.solve_part_1(lines);
        assert_eq!(result, "241344943");
    }

    #[test]
    fn test_part_2() {
        let solver = Day7Solver {};
        let lines = lines_from_file("inputs/2023-day07.txt");
        let result = solver.solve_part_2(lines);
        assert_eq!(result, "243101568");
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
enum PokerLevel {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, PartialOrd, Debug, Copy, Clone)]
enum CardRank {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardRank {
    fn parse_rank(value: String, rule_set: RuleSet) -> Self {
        match value.as_str() {
            "J" => match rule_set {
                RuleSet::Simple => Self::Jack,
                RuleSet::WithJokers => Self::Joker,
            },
            "2" => Self::Two,
            "3" => Self::Three,
            "4" => Self::Four,
            "5" => Self::Five,
            "6" => Self::Six,
            "7" => Self::Seven,
            "8" => Self::Eight,
            "9" => Self::Nine,
            "T" => Self::Ten,
            "Q" => Self::Queen,
            "K" => Self::King,
            "A" => Self::Ace,
            _ => panic!("invalid value for card {value}"),
        }
    }
}

// TODO: Use &str and add lifetimes
#[derive(PartialEq, Debug, Eq)]
struct CamelPokerHand {
    hand: Vec<CardRank>,
    level: PokerLevel,
}

enum RuleSet {
    Simple,
    WithJokers,
}

impl CamelPokerHand {
    fn new_with_jokers(hand: String) -> Self {
        if hand.len() != 5 {
            panic!("poker hand must have 5 cards, found {}", hand.len())
        }
        let mut freq: HashMap<char, u8> = HashMap::new();

        let mut cards = vec![];
        for card in hand.chars() {
            let card_count = freq.entry(card).or_default();
            *card_count += 1;
            cards.push(CardRank::parse_rank(card.to_string(), WithJokers));
        }

        let num_jokers =  freq.remove(&'J').unwrap_or_default();

        if num_jokers == 5 {
            return Self{hand: cards, level: PokerLevel::FiveOfAKind}
        }

        let mut values = freq.values().collect::<Vec<_>>();
        values.sort();
        values.reverse();

        let top_card = *values[0] + num_jokers;

        let level = if top_card == 5 {
            PokerLevel::FiveOfAKind
        } else if top_card == 4 {
            PokerLevel::FourOfAKind
        } else if top_card == 3 && *values[1] == 2 {
            PokerLevel::FullHouse
        } else if top_card == 3 && *values[1] == 1 {
            PokerLevel::ThreeOfAKind
        } else if top_card == 2 && *values[1] == 2 {
            PokerLevel::TwoPair
        } else if top_card == 2 && *values[1] == 1 {
            PokerLevel::OnePair
        } else {
            PokerLevel::HighCard
        };

        Self { hand: cards, level }
    }
    fn new_simple(hand: String) -> Self {
        if hand.len() != 5 {
            panic!("poker hand must have 5 cards, found {}", hand.len())
        }
        let mut freq: HashMap<char, u8> = HashMap::new();

        let mut cards = vec![];
        for card in hand.chars() {
            let card_count = freq.entry(card).or_default();
            *card_count += 1;
            cards.push(CardRank::parse_rank(card.to_string(), Simple));
        }

        let mut values = freq.values().collect::<Vec<_>>();
        values.sort();
        values.reverse();

        let level = if *values[0] == 5 {
            PokerLevel::FiveOfAKind
        } else if *values[0] == 4 {
            PokerLevel::FourOfAKind
        } else if *values[0] == 3 && *values[1] == 2 {
            PokerLevel::FullHouse
        } else if *values[0] == 3 && *values[1] == 1 {
            PokerLevel::ThreeOfAKind
        } else if *values[0] == 2 && *values[1] == 2 {
            PokerLevel::TwoPair
        } else if *values[0] == 2 && *values[1] == 1 {
            PokerLevel::OnePair
        } else {
            PokerLevel::HighCard
        };

        Self { hand: cards, level }
    }

    fn new_with_rules(hand: String, rule_set: RuleSet) -> Self {
        match rule_set {
            RuleSet::Simple => Self::new_simple(hand),
            RuleSet::WithJokers => Self::new_with_jokers(hand),
        }
    }

    fn new(hand: String) -> Self {
        Self::new_simple(hand)
    }
}

struct CamelPokerRound {
    hand: CamelPokerHand,
    bid: usize,
}

impl PartialOrd for CamelPokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Check the strength of the hand first
        if self.level != other.level {
            return self.level.partial_cmp(&other.level);
        }
        // On ties, go by each
        for (self_card, other_card) in zip(self.hand.clone(), other.hand.clone()) {
            let cmp = self_card.partial_cmp(&other_card);
            if cmp != Some(Ordering::Equal) {
                return cmp;
            }
        }
        Some(Ordering::Equal)
    }
}

impl Ord for CamelPokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        self.partial_cmp(other).unwrap()
    }
}
