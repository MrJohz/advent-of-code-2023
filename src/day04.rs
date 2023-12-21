use std::{
    collections::VecDeque,
    simd::{prelude::*, u8x16},
};

use arrayvec::ArrayVec;
use memchr::memchr_iter;

#[derive(Debug)]
struct Winners {
    winners: u8x16,
    length: usize,
}

impl Winners {
    fn new() -> Self {
        Self {
            winners: u8x16::splat(0),
            length: 0,
        }
    }

    fn push(&mut self, winner: u8) {
        self.winners[self.length] = winner;
        self.length += 1;
    }

    fn matches(&self, card: u8) -> bool {
        self.winners.simd_eq(u8x16::splat(card)).any()
    }
}

#[derive(Debug)]
struct Game {
    winners: Winners,
    card: ArrayVec<u8, 25>,
}

impl Game {
    fn from_row(row: &[u8]) -> Self {
        let mut winners = Winners::new();
        let mut idx = memchr::memchr(b':', row).unwrap() + 2;
        while row[idx] != b'|' {
            winners.push(parse_number(&row[idx..], 2));
            idx += 3;
        }
        idx += 1;
        let mut card = ArrayVec::new();
        while idx < row.len() {
            idx += 1;
            card.push(parse_number(&row[idx..], 2));
            idx += 2;
        }

        Self { winners, card }
    }

    fn matches(&self) -> u8 {
        self.card
            .iter()
            .filter(|value| self.winners.matches(**value))
            .count() as u8
    }
}

fn games(input: &[u8]) -> impl Iterator<Item = Game> + '_ {
    let mut last_start = 0;
    memchr_iter(b'\n', input).map(move |idx| {
        let game = Game::from_row(&input[last_start..idx]);
        last_start = idx + 1;
        game
    })
}

pub fn day4_part1(input: &[u8]) -> u32 {
    games(input)
        .map(|game| {
            let wins = game.matches();
            if wins > 0 {
                2_u32.pow((wins - 1).into())
            } else {
                0
            }
        })
        .sum()
}

pub fn day4_part2(input: &[u8]) -> u32 {
    let mut future_wins = VecDeque::<u32>::new();
    games(input)
        .map(|game| {
            let win_count = game.matches();
            let scratchcards = future_wins.pop_front().unwrap_or(0) + 1;
            if future_wins.len() < win_count as usize {
                future_wins.resize(win_count as usize, 0);
            }
            for i in 0..win_count {
                future_wins[i as usize] += scratchcards;
            }
            scratchcards
        })
        .sum()
}

fn parse_number(input: &[u8], count: usize) -> u8 {
    let mut number = 0;
    let mut i = 0;
    while input[i] == b' ' {
        i += 1;
    }
    while i < count {
        number = number * 10 + (input[i] - b'0');
        i += 1;
    }
    number
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_parsing_number_produces_correct_number() {
        assert_eq!(parse_number(b"1", 1), 1,);
        assert_eq!(parse_number(b"123", 3), 123);
        assert_eq!(parse_number(b"213", 3), 213);
        assert_eq!(parse_number(b"12", 2), 12);
    }

    #[test]
    fn test_parsing_number_ignores_preceding_whitespace() {
        assert_eq!(parse_number(b"  1", 3), 1,);
        assert_eq!(parse_number(b" 123", 4), 123);
        assert_eq!(parse_number(b"313", 2), 31);
    }

    #[test]
    fn test_day4_part1_example() {
        let input = utils::load_example(4);
        assert_eq!(day4_part1(&input), 13);
    }

    #[test]
    fn test_day4_part2_example() {
        let input = utils::load_example(4);
        assert_eq!(day4_part2(&input), 30);
    }

    #[test]
    fn test_day4_part1_real() {
        let input = utils::load_real(4);
        assert_eq!(day4_part1(&input), 21105);
    }

    #[test]
    fn test_day4_part2_real() {
        let input = utils::load_real(4);
        assert_eq!(day4_part2(&input), 5329815);
    }
}
