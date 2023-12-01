pub fn day1_part1(input: &[u8]) -> u32 {
    input
        .split(|b| *b == b'\n')
        .map(|line| calibration_value(line, parse_only_digits))
        .sum()
}

pub fn day1_part2(input: &[u8]) -> u32 {
    input
        .split(|b| *b == b'\n')
        .map(|line| calibration_value(line, parse_named_numbers))
        .sum()
}

fn calibration_value(line: &[u8], parser: impl Fn(&[u8], &mut usize) -> Option<u8>) -> u32 {
    let mut pos = 0;
    let start = parser(line, &mut pos).unwrap();
    let mut end = start;
    while let Some(number) = parser(line, &mut pos) {
        end = number;
    }
    (start * 10 + end).into()
}

#[inline(always)]
fn parse_only_digits(line: &[u8], pos: &mut usize) -> Option<u8> {
    while *pos < line.len() {
        *pos += 1;
        if let c @ b'0'..=b'9' = line[*pos - 1] {
            return Some(c - b'0');
        }
    }

    None
}

#[inline(always)]
fn parse_named_numbers(line: &[u8], pos: &mut usize) -> Option<u8> {
    while *pos < line.len() {
        *pos += 1;
        match line[*pos - 1] {
            c @ b'0'..=b'9' => return Some(c - b'0'),
            b'o' => {
                if line.len() > *pos + 1 && &line[*pos..=*pos + 1] == b"ne" {
                    *pos += 1;
                    return Some(1);
                }
            }
            b't' => {
                if line.len() > *pos + 1 && &line[*pos..=*pos + 1] == b"wo" {
                    *pos += 1;
                    return Some(2);
                } else if line.len() > *pos + 3 && &line[*pos..=*pos + 3] == b"hree" {
                    *pos += 3;
                    return Some(3);
                }
            }
            b'f' => {
                if line.len() > *pos + 2 && &line[*pos..=*pos + 2] == b"our" {
                    *pos += 3;
                    return Some(4);
                } else if line.len() > *pos + 2 && &line[*pos..=*pos + 2] == b"ive" {
                    *pos += 2;
                    return Some(5);
                }
            }
            b's' => {
                if line.len() > *pos + 1 && &line[*pos..=*pos + 1] == b"ix" {
                    *pos += 2;
                    return Some(6);
                } else if line.len() > *pos + 3 && &line[*pos..=*pos + 3] == b"even" {
                    *pos += 3;
                    return Some(7);
                }
            }
            b'e' => {
                if line.len() > *pos + 3 && &line[*pos..=*pos + 3] == b"ight" {
                    *pos += 3;
                    return Some(8);
                }
            }
            b'n' => {
                if line.len() > *pos + 2 && &line[*pos..=*pos + 2] == b"ine" {
                    *pos += 2;
                    return Some(9);
                }
            }
            _ => {}
        }
    }

    None
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn parsing_standard_numbers_produces_correct_values() {
        assert_eq!(parse_named_numbers(b"1", &mut 0), Some(1));
        assert_eq!(parse_named_numbers(b"2", &mut 0), Some(2));
        assert_eq!(parse_named_numbers(b"3", &mut 0), Some(3));
        assert_eq!(parse_named_numbers(b"4", &mut 0), Some(4));
        assert_eq!(parse_named_numbers(b"5", &mut 0), Some(5));
        assert_eq!(parse_named_numbers(b"6", &mut 0), Some(6));
        assert_eq!(parse_named_numbers(b"7", &mut 0), Some(7));
        assert_eq!(parse_named_numbers(b"8", &mut 0), Some(8));
        assert_eq!(parse_named_numbers(b"9", &mut 0), Some(9));
        assert_eq!(parse_named_numbers(b"0", &mut 0), Some(0));
    }

    #[test]
    fn parsing_named_numbers_produces_correct_values() {
        assert_eq!(parse_named_numbers(b"one", &mut 0), Some(1));
        assert_eq!(parse_named_numbers(b"two", &mut 0), Some(2));
        assert_eq!(parse_named_numbers(b"three", &mut 0), Some(3));
        assert_eq!(parse_named_numbers(b"four", &mut 0), Some(4));
        assert_eq!(parse_named_numbers(b"five", &mut 0), Some(5));
        assert_eq!(parse_named_numbers(b"six", &mut 0), Some(6));
        assert_eq!(parse_named_numbers(b"seven", &mut 0), Some(7));
        assert_eq!(parse_named_numbers(b"eight", &mut 0), Some(8));
        assert_eq!(parse_named_numbers(b"nine", &mut 0), Some(9));
    }

    #[test]
    fn parsing_partial_named_numbers_produces_no_results() {
        assert_eq!(parse_named_numbers(b"on", &mut 0), None);
        assert_eq!(parse_named_numbers(b"tw", &mut 0), None);
        assert_eq!(parse_named_numbers(b"t", &mut 0), None);
    }

    #[test]
    fn parsing_named_numbers_consumes_the_correct_number_of_bytes() {
        let mut pos = 0;
        let text = b"onetwothreefourfivesixseveneightnine";
        parse_named_numbers(text, &mut pos);
        assert_eq!(pos, 2);
        parse_named_numbers(text, &mut pos);
        assert_eq!(pos, 5);
        parse_named_numbers(text, &mut pos);
        assert_eq!(pos, 10);
        parse_named_numbers(text, &mut pos);
        assert_eq!(pos, 15);
        parse_named_numbers(text, &mut pos);
        assert_eq!(pos, 18);
        parse_named_numbers(text, &mut pos);
        assert_eq!(pos, 22);
        parse_named_numbers(text, &mut pos);
        assert_eq!(pos, 26);
        parse_named_numbers(text, &mut pos);
        assert_eq!(pos, 31);
        parse_named_numbers(text, &mut pos);
        assert_eq!(pos, 35);
    }

    #[test]
    fn parsing_nonsense_digits_consumes_the_correct_number_of_bytes() {
        let mut pos = 0;
        let text = b"otwtwo";
        let parsed = parse_named_numbers(text, &mut pos);
        assert_eq!(pos, 5);
        assert_eq!(parsed, Some(2));
    }

    #[test]
    fn parsing_conjoined_works_successfully() {
        let mut pos = 0;
        let text = b"oneight";
        let parsed = parse_named_numbers(text, &mut pos);
        assert_eq!(pos, 2);
        assert_eq!(parsed, Some(1));
        let parsed = parse_named_numbers(text, &mut pos);
        assert_eq!(pos, 6);
        assert_eq!(parsed, Some(8));
    }

    #[test]
    fn calibration_value_produces_expected_result_with_digits() {
        assert_eq!(calibration_value(b"1234", parse_only_digits), 14);
        assert_eq!(calibration_value(b"91212129", parse_only_digits), 99);
        assert_eq!(calibration_value(b"1abc2", parse_only_digits), 12);
        assert_eq!(calibration_value(b"1", parse_only_digits), 11);
    }

    #[test]
    fn calibration_value_produces_expected_result_with_named_numbers() {
        assert_eq!(calibration_value(b"two1nine", parse_named_numbers), 29);
        assert_eq!(calibration_value(b"rkonedbbf9hq", parse_named_numbers), 19);
        assert_eq!(calibration_value(b"1", parse_named_numbers), 11);
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
