pub fn day1_part1(input: &[u8]) -> u32 {
    input
        .split(|b| *b == b'\n')
        .map(|line| calibration_value(line, parse_digits_forwards, parse_digits_backwards))
        .sum()
}

pub fn day1_part2(input: &[u8]) -> u32 {
    input
        .split(|b| *b == b'\n')
        .map(|line| calibration_value(line, parse_words_forwards, parse_words_backwards))
        .sum()
}

#[inline(always)]
fn calibration_value(
    line: &[u8],
    parse_forwards: impl Fn(&[u8]) -> u8,
    parse_backwards: impl Fn(&[u8]) -> u8,
) -> u32 {
    let start = parse_forwards(line);
    let end = parse_backwards(line);
    (start * 10 + end).into()
}

#[inline(always)]
fn parse_digits_forwards(line: &[u8]) -> u8 {
    let mut pos = 0;
    loop {
        if let c @ b'1'..=b'9' = line[pos] {
            return c - b'0';
        }

        pos += 1;
    }
}

#[inline(always)]
fn parse_digits_backwards(line: &[u8]) -> u8 {
    let mut pos = line.len();
    loop {
        pos -= 1;
        if let c @ b'1'..=b'9' = line[pos] {
            return c - b'0';
        }
    }
}

#[inline(always)]
fn parse_words_forwards(line: &[u8]) -> u8 {
    let mut pos = 0;
    loop {
        match line[pos] {
            c @ b'1'..=b'9' => {
                return c - b'0';
            }
            _ => {
                if line[pos..].starts_with(b"one") {
                    return 1;
                } else if line[pos..].starts_with(b"two") {
                    return 2;
                } else if line[pos..].starts_with(b"three") {
                    return 3;
                } else if line[pos..].starts_with(b"four") {
                    return 4;
                } else if line[pos..].starts_with(b"five") {
                    return 5;
                } else if line[pos..].starts_with(b"six") {
                    return 6;
                } else if line[pos..].starts_with(b"seven") {
                    return 7;
                } else if line[pos..].starts_with(b"eight") {
                    return 8;
                } else if line[pos..].starts_with(b"nine") {
                    return 9;
                } else {
                    pos += 1;
                }
            }
        }
    }
}

#[inline(always)]
fn parse_words_backwards(line: &[u8]) -> u8 {
    let mut pos = line.len();
    loop {
        match line[pos - 1] {
            c @ b'1'..=b'9' => {
                return c - b'0';
            }
            _ => {
                if line[..pos].ends_with(b"one") {
                    return 1;
                } else if line[..pos].ends_with(b"two") {
                    return 2;
                } else if line[..pos].ends_with(b"three") {
                    return 3;
                } else if line[..pos].ends_with(b"four") {
                    return 4;
                } else if line[..pos].ends_with(b"five") {
                    return 5;
                } else if line[..pos].ends_with(b"six") {
                    return 6;
                } else if line[..pos].ends_with(b"seven") {
                    return 7;
                } else if line[..pos].ends_with(b"eight") {
                    return 8;
                } else if line[..pos].ends_with(b"nine") {
                    return 9;
                } else {
                    pos -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn parsing_standard_numbers_produces_correct_values() {
        assert_eq!(parse_words_forwards(b"1"), 1);
        assert_eq!(parse_words_forwards(b"2"), 2);
        assert_eq!(parse_words_forwards(b"3"), 3);
        assert_eq!(parse_words_forwards(b"4"), 4);
        assert_eq!(parse_words_forwards(b"5"), 5);
        assert_eq!(parse_words_forwards(b"6"), 6);
        assert_eq!(parse_words_forwards(b"7"), 7);
        assert_eq!(parse_words_forwards(b"8"), 8);
        assert_eq!(parse_words_forwards(b"9"), 9);
    }

    #[test]
    fn parsing_named_numbers_produces_correct_values() {
        assert_eq!(parse_words_forwards(b"one"), 1);
        assert_eq!(parse_words_forwards(b"two"), 2);
        assert_eq!(parse_words_forwards(b"three"), 3);
        assert_eq!(parse_words_forwards(b"four"), 4);
        assert_eq!(parse_words_forwards(b"five"), 5);
        assert_eq!(parse_words_forwards(b"six"), 6);
        assert_eq!(parse_words_forwards(b"seven"), 7);
        assert_eq!(parse_words_forwards(b"eight"), 8);
        assert_eq!(parse_words_forwards(b"nine"), 9);
    }

    #[test]
    fn parsing_standard_numbers_backwards_produces_correct_values() {
        assert_eq!(parse_words_backwards(b"1"), 1);
        assert_eq!(parse_words_backwards(b"2"), 2);
        assert_eq!(parse_words_backwards(b"3"), 3);
        assert_eq!(parse_words_backwards(b"4"), 4);
        assert_eq!(parse_words_backwards(b"5"), 5);
        assert_eq!(parse_words_backwards(b"6"), 6);
        assert_eq!(parse_words_backwards(b"7"), 7);
        assert_eq!(parse_words_backwards(b"8"), 8);
        assert_eq!(parse_words_backwards(b"9"), 9);
    }

    #[test]
    fn parsing_named_numbers_backwards_produces_correct_values() {
        assert_eq!(parse_words_backwards(b"one"), 1);
        assert_eq!(parse_words_backwards(b"two"), 2);
        assert_eq!(parse_words_backwards(b"three"), 3);
        assert_eq!(parse_words_backwards(b"four"), 4);
        assert_eq!(parse_words_backwards(b"five"), 5);
        assert_eq!(parse_words_backwards(b"six"), 6);
        assert_eq!(parse_words_backwards(b"seven"), 7);
        assert_eq!(parse_words_backwards(b"eight"), 8);
        assert_eq!(parse_words_backwards(b"nine"), 9);
    }

    #[test]
    fn calibration_value_produces_expected_result_with_digits() {
        assert_eq!(
            calibration_value(b"1234", parse_digits_forwards, parse_digits_backwards),
            14
        );
        assert_eq!(
            calibration_value(b"91212129", parse_digits_forwards, parse_digits_backwards),
            99
        );
        assert_eq!(
            calibration_value(b"1abc2", parse_digits_forwards, parse_digits_backwards),
            12
        );
        assert_eq!(
            calibration_value(b"1", parse_digits_forwards, parse_digits_backwards),
            11
        );
    }

    #[test]
    fn calibration_value_produces_expected_result_with_named_numbers() {
        assert_eq!(
            calibration_value(b"two1nine", parse_words_forwards, parse_words_backwards),
            29
        );
        assert_eq!(
            calibration_value(b"rkonedbbf9hq", parse_words_forwards, parse_words_backwards),
            19
        );
        assert_eq!(
            calibration_value(b"1", parse_words_forwards, parse_words_backwards),
            11
        );
        assert_eq!(
            calibration_value(b"one", parse_words_forwards, parse_words_backwards),
            11
        );
        assert_eq!(
            calibration_value(b"1two", parse_words_forwards, parse_words_backwards),
            12
        );
        assert_eq!(
            calibration_value(b"nine9", parse_words_forwards, parse_words_backwards),
            99
        );
    }

    #[test]
    fn day1_part1_sums_all_lines() {
        assert_eq!(day1_part1(b"14"), 14);
        assert_eq!(day1_part1(b"14\n23"), 14 + 23);
    }

    #[test]
    fn test_day1_part1() {
        let input = utils::load_example(1);
        assert_eq!(day1_part1(&input), 142);
    }

    #[test]
    fn test_day1_part2_original_example() {
        let input = utils::load_example(1);
        assert_eq!(day1_part2(&input), 142);
    }

    #[test]
    fn test_day1_part2_second_example() {
        let input = utils::load_example_with_suffix(1, "part2");
        assert_eq!(day1_part2(&input), 281);
    }

    #[test]
    fn test_day1_part1_real() {
        let input = utils::load_real(1);
        assert_eq!(day1_part1(&input), 53921);
    }

    #[test]
    fn test_day1_part2_real() {
        let input = utils::load_real(1);
        assert_eq!(day1_part2(&input), 54676);
    }
}
