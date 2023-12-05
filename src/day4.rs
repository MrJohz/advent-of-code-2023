use arrayvec::ArrayVec;
use memchr::memchr_iter;

#[derive(Debug)]
struct Game {
    winners: ArrayVec<u8, 10>,
    card: ArrayVec<u8, 25>,
}

impl Game {
    fn from_row(row: &[u8]) -> Self {
        let mut winners = ArrayVec::new();
        let mut idx = 10;
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

    fn score(&self) -> u32 {
        let mut score = 0;
        for value in &self.card {
            if self.winners.contains(value) {
                score = if score == 0 { 1 } else { score * 2 }
            }
        }
        score
    }
}

pub fn day4_part1(input: &[u8]) -> u32 {
    let mut last_start = 0;
    memchr_iter(b'\n', input)
        .map(|idx| {
            let game = Game::from_row(&input[last_start..idx]);
            last_start = idx + 1;
            game
        })
        .map(|game| game.score())
        .sum()
}

pub fn day4_part2(_input: &[u8]) -> u32 {
    0
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

    // #[test]
    // fn test_day4_part1_example() {
    //     let input = utils::load_example(4);
    //     assert_eq!(day4_part1(&input), 4361);
    // }

    // #[test]
    // fn test_day4_part2_example() {
    //     let input = utils::load_example(4);
    //     assert_eq!(day4_part2(&input), 467835);
    // }

    #[test]
    fn test_day4_part1_real() {
        let input = utils::load_real(4);
        assert_eq!(day4_part1(&input), 21105);
    }

    // #[test]
    // fn test_day4_part2_real() {
    //     let input = utils::load_real(4);
    //     assert_eq!(day4_part2(&input), 82818007);
    // }
}
