use std::iter::once;

use memchr::memchr_iter;

pub fn day9_part1(input: &[u8]) -> i32 {
    let mut changes = Vec::new();
    rows_forwards(input)
        .map(|sequence| {
            changes.clear();
            extrapolate(sequence.collect_into::<Vec<_>>(&mut changes))
        })
        .sum()
}

pub fn day9_part2(input: &[u8]) -> i32 {
    let mut changes = Vec::new();
    rows_backwards(input)
        .map(|sequence| {
            changes.clear();
            extrapolate(sequence.collect_into::<Vec<_>>(&mut changes))
        })
        .sum()
}

fn rows_forwards(input: &[u8]) -> impl Iterator<Item = impl Iterator<Item = i32> + '_> {
    once(0)
        .chain(memchr_iter(b'\n', input).map(|end| end + 1))
        .map_windows(|[start, end]| &input[*start..*end])
        .map(ForwardsHistoryIter::new)
}

fn rows_backwards(input: &[u8]) -> impl Iterator<Item = impl Iterator<Item = i32> + '_> {
    once(0)
        .chain(memchr_iter(b'\n', input).map(|end| end + 1))
        .map_windows(|[start, end]| &input[*start..*end])
        .map(BackwardsHistoryIter::new)
}

fn extrapolate(sequence: &mut [i32]) -> i32 {
    let mut sums = 0;
    for iterations in (0..sequence.len()).rev() {
        let current_value = sequence[0];
        let mut all_equal = true;
        for i in 0..iterations {
            let diff = sequence[i + 1] - sequence[i];
            all_equal = all_equal && sequence[i + 1] == current_value;
            sequence[i] = diff;
        }
        sums += sequence[iterations];
        if all_equal {
            break;
        }
    }

    sums
}

#[derive(Debug, Clone, Copy)]
struct ForwardsHistoryIter<'a> {
    input: &'a [u8],
    index: usize,
}

impl<'a> ForwardsHistoryIter<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { input, index: 0 }
    }
}

impl<'a> Iterator for ForwardsHistoryIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sign = 1;
        let mut value = 0;
        while self.index < self.input.len() {
            match self.input[self.index] {
                c @ b'0'..=b'9' => {
                    value *= 10;
                    value += i32::from(c - b'0');
                }
                b'-' => {
                    sign = -1;
                }
                _ => {
                    self.index += 1;
                    return Some(sign * value);
                }
            }
            self.index += 1;
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
struct BackwardsHistoryIter<'a> {
    input: &'a [u8],
    index: usize,
}

impl<'a> BackwardsHistoryIter<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            // lines always end with a newline, which we want to ignore
            index: input.len() - 1,
        }
    }
}

impl<'a> Iterator for BackwardsHistoryIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            return None;
        }

        let mut sign = 1;
        let mut value = 0;
        let mut position = 1;
        while self.index > 0 {
            self.index -= 1;
            match self.input[self.index] {
                c @ b'0'..=b'9' => {
                    value += i32::from(c - b'0') * position;
                    position *= 10;
                }
                b'-' => {
                    sign = -1;
                }
                _ => {
                    return Some(sign * value);
                }
            }
        }
        Some(sign * value)
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::utils;

    use test_case::test_case;

    #[test_case(b"1 2 3 4 5 6\n" => vec![1, 2, 3, 4, 5, 6]; "positive numbers")]
    #[test_case(b"-1 -2 -3 -4 -5 -6\n" => vec![-1, -2, -3, -4, -5, -6]; "negative numbers")]
    #[test_case(b"123 223 323\n" => vec![123, 223, 323]; "positive multi-digit numbers")]
    #[test_case(b"-123 -223 -323\n" => vec![-123, -223, -323]; "negative multi-digit numbers")]
    fn test_can_parse_history_row_forwards(input: &[u8]) -> Vec<i32> {
        ForwardsHistoryIter::new(input).collect()
    }

    #[test_case(b"1 2 3 4 5 6\n" => vec![6, 5, 4, 3, 2, 1]; "positive numbers")]
    #[test_case(b"-1 -2 -3 -4 -5 -6\n" => vec![-6, -5, -4, -3, -2, -1]; "negative numbers")]
    #[test_case(b"123 223 323\n" => vec![323, 223, 123]; "positive multi-digit numbers")]
    #[test_case(b"-123 -223 -323\n" => vec![-323, -223, -123]; "negative multi-digit numbers")]
    fn test_can_parse_history_row_backwards(input: &[u8]) -> Vec<i32> {
        BackwardsHistoryIter::new(input).collect()
    }

    #[test_case(vec![1, 3, 6, 10, 15, 21] => 28)]
    #[test_case(vec![0, 3, 6, 9, 12, 15] => 18)]
    #[test_case(vec![10, 13, 16, 21, 30, 45] => 68)]
    fn test_can_extrapolate_next_value_in_sequence(mut input: Vec<i32>) -> i32 {
        extrapolate(&mut input)
    }

    // #[test]
    // fn test_day9_part1_example() {
    //     let input = utils::load_example(9);
    //     assert_eq!(day9_part1(&input), 114);
    // }

    // #[test]
    // fn test_day9_part2_example() {
    //     let input = utils::load_example(9);
    //     assert_eq!(day9_part2(&input), 2);
    // }

    // #[test]
    // fn test_day9_part1_real() {
    //     let input = utils::load_real(9);
    //     assert_eq!(day9_part1(&input), 1969958987);
    // }

    // #[test]
    // fn test_day9_part2_real() {
    //     let input = utils::load_real(9);
    //     assert_eq!(day9_part2(&input), 1068);
    // }
}
