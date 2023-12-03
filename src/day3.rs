#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn up(mut self) -> Self {
        self.y -= 1;
        self
    }

    fn jump_up(mut self, steps: i16) -> Self {
        self.y -= steps;
        self
    }

    fn down(mut self) -> Self {
        self.y += 1;
        self
    }

    fn jump_down(mut self, steps: i16) -> Self {
        self.y += steps;
        self
    }

    fn left(mut self) -> Self {
        self.x -= 1;
        self
    }

    fn jump_left(mut self, steps: i16) -> Self {
        self.x -= steps;
        self
    }

    fn right(mut self) -> Self {
        self.x += 1;
        self
    }

    fn jump_right(mut self, steps: i16) -> Self {
        self.x += steps;
        self
    }
}

#[derive(Debug)]
struct Grid<'a> {
    grid: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> Grid<'a> {
    fn new(grid: &'a [u8]) -> Self {
        let width = memchr::memchr(b'\n', grid).unwrap();
        let height = grid.len() / (width + 1);
        Self {
            grid,
            width,
            height,
        }
    }

    fn get(&self, index: Position) -> &u8 {
        if index.x < 0 || index.y < 0 {
            &b'.'
        } else {
            let x = usize::try_from(index.x).unwrap();
            let y = usize::try_from(index.y).unwrap();
            if x >= self.width || y >= self.height {
                &b'.'
            } else {
                &self.grid[y * (self.width + 1) + x]
            }
        }
    }

    fn numbers(&self) -> impl Iterator<Item = (u32, Region)> + '_ {
        Numbers {
            input: self.grid,
            pos: 0,
        }
        .map(|(n, (index, length))| {
            (
                n,
                Region::new(
                    Position::new(
                        (index % (self.width + 1)).try_into().unwrap(),
                        (index / (self.width + 1)).try_into().unwrap(),
                    ),
                    length,
                ),
            )
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Region {
    index: Position,
    length: usize,
}

impl Region {
    fn new(index: Position, length: usize) -> Self {
        Self { index, length }
    }
}

pub fn day3_part1(input: &[u8]) -> u32 {
    let grid = Grid::new(input);
    grid.numbers()
        .filter(|(_, region)| touching_symbol(&grid, *region))
        .map(|(n, _)| n)
        .sum()
}

pub fn day3_part2(_: &[u8]) -> u32 {
    0
}

fn parse_number(input: &[u8]) -> (u32, usize) {
    let mut pos = 0;
    let mut sum: u32 = 0;
    while let c @ b'0'..=b'9' = input[pos] {
        sum *= 10;
        sum += u32::from(c - b'0');
        pos += 1;
    }

    (sum, pos)
}

#[derive(Debug)]
struct Numbers<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for Numbers<'a> {
    type Item = (u32, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.input.get(self.pos) {
                Some(b'0'..=b'9') => {
                    let (value, length) = parse_number(&self.input[self.pos..]);
                    let result = Some((value, (self.pos, length)));
                    self.pos += length;
                    break result;
                }
                None => break None,
                _ => self.pos += 1,
            }
        }
    }
}

fn is_symbol(char: &u8) -> bool {
    !matches!(char, b'.' | b'0'..=b'9')
}

fn touching_symbol(input: &Grid<'_>, region: Region) -> bool {
    let Region { index, length } = region;

    let mut top_row = index.up().left();
    let mut bottom_row = index.down().left();
    for _ in 0..(length + 2) {
        if is_symbol(input.get(top_row)) || is_symbol(input.get(bottom_row)) {
            return true;
        }
        top_row = top_row.right();
        bottom_row = bottom_row.right();
    }

    is_symbol(input.get(index.left()))
        || is_symbol(input.get(index.jump_right(length.try_into().unwrap())))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_parsing_number_returns_the_correct_number() {
        assert_eq!(parse_number(b"1\n"), (1, 1));
        assert_eq!(parse_number(b"123\n"), (123, 3));
        assert_eq!(parse_number(b"313."), (313, 3));
        assert_eq!(parse_number(b"12*"), (12, 2));
    }

    // #[test]
    // fn test_parsing_all_numbers_returns_iterator_of_numbers() {
    //     let mut numbers = find_numbers(b"123..345**6\n123\n");
    //     assert_eq!(numbers.next(), Some((123, Region::new(0, 3))));
    //     assert_eq!(numbers.next(), Some((345, Region::new(5, 3))));
    //     assert_eq!(numbers.next(), Some((6, Region::new(10, 1))));
    //     assert_eq!(numbers.next(), Some((123, Region::new(12, 3))));
    //     assert_eq!(numbers.next(), None);
    // }

    #[test]
    fn test_day3_part1_example() {
        let input = utils::load_example(3);
        assert_eq!(day3_part1(&input), 4361);
    }

    // #[test]
    // fn test_day3_part2_example() {
    //     let input = utils::load_example(3);
    //     assert_eq!(day3_part2(&input), 2286);
    // }

    #[test]
    fn test_day3_part1_real() {
        let input = utils::load_real(3);
        assert_eq!(day3_part1(&input), 539637);
    }

    // #[test]
    // fn test_day3_part2_real() {
    //     let input = utils::load_real(3);
    //     assert_eq!(day3_part2(&input), 78669);
    // }
}
