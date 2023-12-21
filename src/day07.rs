use std::{fmt::Debug, iter::once};

pub fn day7_part1(input: &[u8]) -> u64 {
    let mut hands = parse_lines::<false>(input).collect::<Vec<_>>();
    hands.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, (_, bet))| acc + bet * (index + 1) as u64)
}

pub fn day7_part2(input: &[u8]) -> u64 {
    let mut hands = parse_lines::<true>(input).collect::<Vec<_>>();
    hands.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, (_, bet))| acc + bet * (index + 1) as u64)
}

fn parse_lines<const JOKERS: bool>(input: &[u8]) -> impl Iterator<Item = (Hand, u64)> + '_ {
    once(0)
        .chain(memchr::memchr_iter(b'\n', input).map(|index| index + 1))
        .map_windows(|[start, end]| parse_line::<JOKERS>(&input[*start..*end - 1]))
}

fn parse_line<const JOKERS: bool>(input: &[u8]) -> (Hand, u64) {
    let hand = parse_hand::<JOKERS>(input);

    let mut bet = 0;
    let mut index = 6;
    while index < input.len() {
        bet *= 10;
        bet += (input[index] - b'0') as u64;
        index += 1;
    }

    (hand, bet)
}

fn parse_hand<const JOKERS: bool>(input: &[u8]) -> Hand {
    let mut cards = [0_u8; 5];
    let mut counts = [0_u8; 13];
    let mut largest = (0, 0);
    let mut second = (0, 0);
    let mut jokers = 0;
    for entry in cards.iter_mut().enumerate() {
        *entry.1 = match input[entry.0] {
            c @ b'2'..=b'9' => c - b'1',
            b'T' => 9,
            b'J' => {
                if JOKERS {
                    0
                } else {
                    10
                }
            }
            b'Q' => 11,
            b'K' => 12,
            b'A' => 13,
            _ => unreachable!("Invalid card {:?}", input[entry.0] as char),
        };
        if JOKERS && *entry.1 == 0 {
            jokers += 1;
        } else {
            let count = counts.get_mut(*entry.1 as usize - 1).unwrap();
            *count += 1;
            if *entry.1 == largest.0 && *count > largest.1 {
                largest.1 = *count;
            } else if *count > largest.1 {
                second = largest;
                largest = (*entry.1, *count);
            } else if *count > second.1 {
                second = (*entry.1, *count);
            }
        }
    }

    let kind = match (largest.1 + jokers, second.1) {
        (1, _) => HandKind::HighCard,
        (2, 1) => HandKind::OnePair,
        (2, 2) => HandKind::TwoPairs,
        (3, 1) => HandKind::ThreeOfAKind,
        (3, 2) => HandKind::FullHouse,
        (4, _) => HandKind::FourOfAKind,
        (5, _) => HandKind::FiveOfAKind,
        _ => unreachable!("Invalid hand"),
    };

    Hand { cards, kind }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    kind: HandKind,
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|&card| match card {
                0 => '*',
                c @ 1..=8 => (c + b'1') as char,
                9 => 'T',
                10 => 'J',
                11 => 'Q',
                12 => 'K',
                13 => 'A',
                _ => unreachable!("Invalid card {:?}", card),
            })
            .collect::<String>();
        write!(f, "{:?} ({:?})", self.kind, cards)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils;

    use test_case::test_case;

    #[test_case([b'2', b'2', b'2', b'2', b'2'], HandKind::FiveOfAKind ; "five of a kind")]
    #[test_case([b'2', b'2', b'2', b'2', b'5'], HandKind::FourOfAKind ; "four of a kind")]
    #[test_case([b'2', b'5', b'2', b'2', b'2'], HandKind::FourOfAKind ; "four of a kind reordered")]
    #[test_case([b'2', b'2', b'2', b'9', b'5'], HandKind::ThreeOfAKind ; "three of a kind")]
    #[test_case([b'9', b'6', b'2', b'2', b'2'], HandKind::ThreeOfAKind ; "three of a kind reordered")]
    #[test_case([b'5', b'2', b'2', b'2', b'5'], HandKind::FullHouse ; "full house")]
    #[test_case([b'2', b'2', b'9', b'5', b'5'], HandKind::TwoPairs ; "two pairs")]
    #[test_case([b'2', b'2', b'9', b'6', b'5'], HandKind::OnePair ; "one pair")]
    #[test_case([b'2', b'3', b'4', b'5', b'8'], HandKind::HighCard ; "high card")]
    fn test_can_get_hand_cards(input: [u8; 5], expected: HandKind) {
        let hand = parse_hand::<false>(input.as_ref());
        assert_eq!(hand.kind, expected);
    }

    #[test_case([b'2', b'3', b'4', b'5', b'J'], HandKind::OnePair ; "joker converts to pair")]
    #[test_case([b'2', b'2', b'4', b'5', b'J'], HandKind::ThreeOfAKind ; "joker converts to three of a kind")]
    #[test_case([b'2', b'2', b'3', b'3', b'J'], HandKind::FullHouse ; "joker converts to full house")]
    #[test_case([b'2', b'2', b'2', b'3', b'J'], HandKind::FourOfAKind ; "joker converts to four of a kind")]
    #[test_case([b'2', b'2', b'2', b'2', b'J'], HandKind::FiveOfAKind ; "joker converts to five of a kind")]
    #[test_case([b'2', b'3', b'4', b'J', b'J'], HandKind::ThreeOfAKind ; "two jokers converts to three of a kind")]
    #[test_case([b'2', b'2', b'4', b'J', b'J'], HandKind::FourOfAKind ; "two jokers converts to four of a kind")]
    #[test_case([b'2', b'2', b'2', b'J', b'J'], HandKind::FiveOfAKind ; "two jokers converts to five of a kind")]
    #[test_case([b'2', b'3', b'J', b'J', b'J'], HandKind::FourOfAKind ; "three jokers converts to four of a kind")]
    #[test_case([b'2', b'2', b'J', b'J', b'J'], HandKind::FiveOfAKind ; "three jokers converts to five of a kind")]
    #[test_case([b'2', b'J', b'J', b'J', b'J'], HandKind::FiveOfAKind ; "four jokers converts to five of a kind")]
    #[test_case([b'J', b'J', b'J', b'J', b'J'], HandKind::FiveOfAKind ; "five jokers converts to five of a kind")]

    fn parsing_hand_with_jokers_returns_joker_hands(input: [u8; 5], expected: HandKind) {
        let hand = parse_hand::<true>(input.as_ref());
        assert_eq!(hand.kind, expected);
    }

    #[test_case([b'2', b'2', b'2', b'2', b'2'], [b'2', b'2', b'2', b'2', b'3'] ; "five of a kind beats four of a kind")]
    #[test_case([b'2', b'2', b'2', b'2', b'5'], [b'2', b'2', b'2', b'3', b'3'] ; "four of a kind beats full house")]
    #[test_case([b'2', b'2', b'2', b'9', b'5'], [b'2', b'2', b'4', b'4', b'3'] ; "three of a kind beats two pairs")]
    #[test_case([b'A', b'A', b'A', b'A', b'A'], [b'K', b'K', b'K', b'K', b'K'] ; "five of a kind beats five of a kind")]
    #[test_case([b'3', b'6', b'4', b'5', b'2'], [b'3', b'5', b'4', b'6', b'2'] ; "high card beats high card")]
    fn test_card_comparisons(left: [u8; 5], right: [u8; 5]) {
        let left = parse_hand::<false>(left.as_ref());
        let right = parse_hand::<false>(right.as_ref());
        assert!(left > right);
    }

    #[test]
    fn test_day7_part1_example() {
        let input = utils::load_example(7);
        assert_eq!(day7_part1(&input), 6440);
    }

    #[test]
    fn test_day7_part2_reddit_example() {
        let input = utils::load_example_with_suffix(7, "from-reddit");
        assert_eq!(day7_part2(&input), 4657);
    }

    #[test]
    fn test_day7_part2_example() {
        let input = utils::load_example(7);
        assert_eq!(day7_part2(&input), 5905);
    }

    #[test]
    fn test_day7_part1_real() {
        let input = utils::load_real(7);
        assert_eq!(day7_part1(&input), 253603890);
    }

    #[test]
    fn test_day7_part2_real() {
        let input = utils::load_real(7);
        assert_eq!(day7_part2(&input), 253630098);
    }
}
