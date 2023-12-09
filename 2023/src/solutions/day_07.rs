use std::{collections::HashMap, cmp::Ordering};

use aoc_common::{Solution, load};
use itertools::Itertools;

pub struct Day07;

impl Solution for Day07 {
    fn name(&self) -> String {
        "Camel Cards".to_owned()
    }

    fn part_1(&self) -> String {
        let input = load("day_07");
        let mut rounds: Vec<Hand> = input.lines()
            .map(|l| l.into())
            .collect_vec();

        rounds.sort();

        let wins = rounds.iter()
            .enumerate()
            .map(|(i, hand)| (i as isize + 1) * hand.bid)
            .sum::<isize>();

        wins.to_string()
    }

    fn part_2(&self) -> String {
        let input = load("day_07");
        let mut rounds: Vec<Hand> = input.replace('J', "j")
            .lines()
            .map(|l| l.into())
            .collect_vec();

            rounds.sort();

            let wins = rounds.iter()
                .enumerate()
                .map(|(i, hand)| (i as isize + 1) * hand.bid)
                .sum::<isize>();
    
            wins.to_string()
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    bid: isize
}

impl Into<Hand> for &str {
    fn into(self) -> Hand {
        let (cards_str, bid_str) = self.split_whitespace()
            .collect_tuple::<(&str, &str)>()
            .unwrap();

        let bid = bid_str.parse().unwrap();
        let cards = cards_str.chars()
            .map(|c| c.into())
            .collect_vec();

        Hand { cards, bid }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Hand {
    fn get_hand_type(&self) -> HandType {
        let card_frequency = get_cards_frequency(&self.cards);
        let max_frequency = card_frequency.values().max().unwrap_or(&0);

        match max_frequency {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if card_frequency.values().any(|v| v == &2) {
                    return HandType::FullHouse;
                }

                HandType::ThreeOfAKind
            },
            2 => {
                if card_frequency.values().filter(|&&v| v == 2).count() == 2 {
                    return HandType::TwoPair;
                }

                HandType::OnePair
            },
            1 => HandType::HighCard,
            0 => panic!("should not happen"),
            _ => unreachable!()
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_hand_type() == other.get_hand_type() {
            self.cards.cmp(&other.cards)
        } else {
            self.get_hand_type().cmp(&other.get_hand_type())
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
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

impl Into<Card> for char {
    fn into(self) -> Card {
        match self {
            'j' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("{} is an invalid card", self)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_cards_frequency(cards: &Vec<Card>) -> HashMap<Card, isize> {
    let mut map = HashMap::new();

    for card in cards {
        *map.entry(card.clone()).or_insert(0) += 1;
    }

    if let Some(&joker_count) = map.get(&Card::Joker) {
        let find_max = map.iter_mut()
            .filter(|(&c, _)| c != Card::Joker)
            .max_by(|a, b| {
                if a.1 == b.1 {
                    a.0.cmp(b.0)
                } else {
                    a.1.cmp(&b.1)
                }
            })
            .map(|(c, _)| *c);

        if let Some(max_no_joker) = find_max {
            *map.entry(max_no_joker).or_insert(0) += joker_count;
            *map.entry(Card::Joker).or_insert(0) = 0;
        }
    }

    map
}