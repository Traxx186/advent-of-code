use aoc_common::{load, Solution};

pub struct Day04 {}

impl Solution for Day04 {

    fn name(&self) -> String {
        "Giant Squid".to_owned()
    }

    fn part_1(&self) -> String {
        let bingo = Bingo::parse_input(load("day_04"));
        let winning = bingo.solve();

        winning.0[winning.1].calculate_score(winning.2).to_string()
    }

    fn part_2(&self) -> String {
        let bingo = Bingo::parse_input(load("day_04"));
        let winning = bingo.losing_solve();

        winning.0[winning.1].calculate_score(winning.2).to_string()
    }
}

#[derive(Debug, Clone)]
struct Bingo {
    numbers: Vec<u32>,
    cards: Vec<Card>,
    take: u32
}

#[derive(Debug, Clone)]
struct Card {
    data: Vec<Vec<u32>>,
    checked: Vec<Vec<bool>>
}

impl Bingo {

    fn parse_input(input: String) -> Self {
        let mut lines = input.lines();
        let numbers = lines.next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let cards = Card::parse_input(input);

        Bingo { numbers, cards, take: 5 }
    }

    fn solve(self) -> (Vec<Card>, usize, u32) {
        let mut numbers = self.numbers.clone();
        let mut tick = self.cards;
        let mut take = self.take;

        loop {
            for _ in 1..take {
                let num = numbers.remove(0);
                tick = Card::tick(tick, num);

                if let Some(i) = Card::check(tick.clone()).first() {
                    return (tick, *i, num);
                };
            }

            take += 1;
        }
    }

    fn losing_solve(self) -> (Vec<Card>, usize, u32) {
        let mut numbers = self.numbers.clone();
        let mut tick = self.cards;
        let mut take = self.take;
        let mut ret = Vec::new();

        loop {
            for _ in 1..take {
                let num = numbers.remove(0);
                tick = Card::tick(tick, num);

                let check = Card::check(tick.clone());
                for i in check.clone() {
                    if !ret.contains(&i) {
                        ret.push(i);
                    }
                }

                if ret.len() == tick.len() {
                    return (tick, *ret.last().unwrap(), num);
                }
            }

            take += 1;
        }
    }
}

impl Card {

    fn parse_input(input: String) -> Vec<Self> {
        let mut cards = Vec::new();
        let input = input.replace('\r', "");
        let raw_boards = input.split("\n\n").skip(1);

        for board in raw_boards {
            let mut data = Vec::new();
            let mut checked = Vec::new();

            for line in board.lines() {
                let numbers = line.split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

                data.push(numbers.clone());
                checked.push(vec![false; line.len()]);
            }

            cards.push(Card { data, checked });
        }

        cards
    }

    fn tick(data: Vec<Card>, num: u32) -> Vec<Card> {
        let mut data = data;
        for (card_i, card) in data.clone().iter().enumerate() {
            for (row_i, row) in card.data.iter().enumerate() {
                for (column_i, column) in row.iter().enumerate() {
                    if *column == num {
                        data[card_i].checked[row_i][column_i] = true;
                    }
                }
            }
        }

        data
    }

    fn check(cards: Vec<Card>) -> Vec<usize> {
        let mut out = Vec::new();
        for (i, card) in cards.iter().enumerate() {
            let card_row_length = card.data[0].len();
            let card_col_length = card.data.len();

            for row in card.checked.iter() {
                let mut row_count = 0;
                for i in row.iter().take(card_col_length) {
                    if *i {
                        row_count += 1;
                    }
                }

                if row_count == card_row_length {
                    out.push(i);
                }
            }

            for col_i in 0..card_col_length {
                let mut col_count = 0;
                for row_i in 0..card_row_length {
                    if card.checked[row_i][col_i] {
                        col_count += 1;
                    }
                }

                if col_count == card_row_length {
                    out.push(i);
                }
            }
        }

        out
    }

    fn calculate_score(&self, winning: u32) -> usize {
        let mut sum = 0;
        for (row_i, row) in self.data.iter().enumerate() {
            for (column_i, column) in row.iter().enumerate() {
                if !self.checked[row_i][column_i] {
                    sum += *column;
                }
            }
        }

        sum as usize * winning as usize
    }
}