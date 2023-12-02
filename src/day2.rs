use std::hint::unreachable_unchecked;

pub fn day2_part1(input: &[u8]) -> u32 {
    let mut index = 0;
    let mut id_sum = 0;
    while index < input.len() {
        let (id, bytes_read) = id_or_zero(&input[index..]);
        id_sum += id;
        index += bytes_read;
    }
    id_sum
}

pub fn day2_part2(input: &[u8]) -> u32 {
    let mut index = 0;
    let mut power_sum = 0;
    while index < input.len() {
        let (p, bytes_read) = power(&input[index..]);
        power_sum += p;
        index += bytes_read;
    }

    power_sum
}

fn power(input: &[u8]) -> (u32, usize) {
    // skup "Game" header text and id
    let mut index = 5;
    index += find_byte(&input[index..], b':') + 1;
    let ((blue, green, red), bytes_read) = minimal_cubes(&input[index..]);
    let blue: u32 = blue.into();
    let green: u32 = green.into();
    let red: u32 = red.into();
    (blue * green * red, index + bytes_read)
}

fn id_or_zero(input: &[u8]) -> (u32, usize) {
    // skip "Game" header text
    let mut index = 5;
    let (id, bytes_read) = parse_number(&input[index..], b':');
    index += bytes_read + 1;
    let (large, bytes_read) = is_large_game(&input[index..]);
    index += bytes_read;
    (if large { 0 } else { id.into() }, index)
}

fn parse_number(input: &[u8], sep: u8) -> (u8, usize) {
    let mut acc = 0;
    let mut index = 0;
    while input[index] != sep {
        acc *= 10;
        acc += input[index] - b'0';
        index += 1;
    }

    (acc, index + 1)
}

fn minimal_cubes(input: &[u8]) -> ((u8, u8, u8), usize) {
    let mut index = 0;
    let mut blues = 0;
    let mut greens = 0;
    let mut reds = 0;
    loop {
        let (n, bytes) = parse_number(&input[index..], b' ');
        index += bytes;
        match input[index] {
            b'b' => {
                blues = blues.max(n);
                index += 4;
            }
            b'g' => {
                greens = greens.max(n);
                index += 5;
            }
            b'r' => {
                reds = reds.max(n);
                index += 3;
            }
            // safety: this is only safe if the input is guaranteed to match the format of the AOC input
            _ => unsafe { unreachable_unchecked() }, // _ => unreachable!("not a colour {input}@{index}", input = input[index] as char),
        }
        // account for comma/semicolon combo
        match input.get(index) {
            Some(b'\n') | None => break,
            _ => index += 2,
        }
    }

    ((blues, greens, reds), index + 1)
}

fn is_large_game(input: &[u8]) -> (bool, usize) {
    let mut index = 0;
    loop {
        let (n, bytes) = parse_number(&input[index..], b' ');
        index += bytes;
        match input[index] {
            b'b' => {
                if n > 14 {
                    return (true, find_byte(&input[index + 4..], b'\n') + index + 4);
                } else {
                    index += 4
                }
            }
            b'g' => {
                if n > 13 {
                    return (true, find_byte(&input[index + 5..], b'\n') + index + 5);
                } else {
                    index += 5
                }
            }
            b'r' => {
                if n > 12 {
                    return (true, find_byte(&input[index + 3..], b'\n') + index + 3);
                } else {
                    index += 3
                }
            }
            // safety: this is only safe if the input is guaranteed to match the format of the AOC input
            _ => unsafe { unreachable_unchecked() }, // _ => unreachable!("not a colour {input}@{index}", input = input[index] as char),
        }
        // account for comma/semicolon combo
        match input.get(index) {
            Some(b'\n') | None => break,
            _ => index += 2,
        }
    }

    (false, index + 1)
}

fn find_byte(input: &[u8], byte: u8) -> usize {
    // memchr::memchr(byte, input).unwrap_or(input.len() - 100)
    let mut index = 0;
    while index < input.len() && input[index] != byte {
        index += 1;
    }
    index + 1
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn parse_number_parses_numbers() {
        assert_eq!(parse_number(b"1:", b':'), (1, 2));
        assert_eq!(parse_number(b"11:", b':'), (11, 3));
        assert_eq!(parse_number(b"99 123", b' '), (99, 3));
    }

    #[test]
    fn is_large_game_detects_large_games() {
        assert_eq!(is_large_game(b"123 red\n"), (true, 8));
        assert_eq!(is_large_game(b"0 red, 123 green\n"), (true, 17));
        assert_eq!(is_large_game(b"0 red; 123 blue\n"), (true, 16));
    }

    #[test]
    fn minimal_cubes_finds_minimal_cubes() {
        assert_eq!(minimal_cubes(b"123 red\n"), ((0, 0, 123), 8));
        assert_eq!(minimal_cubes(b"1 red, 2 green\n"), ((0, 2, 1), 15));
        assert_eq!(minimal_cubes(b"3 red; 4 red\n"), ((0, 0, 4), 13));
    }

    #[test]
    fn test_day2_part1_example() {
        let input = utils::load_example(2);
        assert_eq!(day2_part1(&input), 8);
    }

    #[test]
    fn test_day2_part2_example() {
        let input = utils::load_example(2);
        assert_eq!(day2_part2(&input), 2286);
    }

    #[test]
    fn test_day2_part1_real() {
        let input = utils::load_real(2);
        assert_eq!(day2_part1(&input), 2283);
    }

    #[test]
    fn test_day2_part2_real() {
        let input = utils::load_real(2);
        assert_eq!(day2_part2(&input), 78669);
    }
}
