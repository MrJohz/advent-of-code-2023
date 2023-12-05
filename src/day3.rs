use arrayvec::ArrayVec;
use memchr::{memchr3_iter, memchr_iter};

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

    fn down(mut self) -> Self {
        self.y += 1;
        self
    }

    fn left(mut self) -> Self {
        self.x -= 1;
        self
    }

    fn right(mut self) -> Self {
        self.x += 1;
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

    fn slice_after(&self, index: Position) -> &[u8] {
        let x = usize::try_from(index.x).unwrap();
        let y = usize::try_from(index.y).unwrap();
        &self.grid[(y * (self.width + 1) + x)..]
    }

    fn symbols(&self) -> impl Iterator<Item = Position> + '_ {
        // Note: this list of symbols was obtained manually by searching through the input text.
        memchr3_iter(b'*', b'@', b'#', self.grid)
            .chain(memchr3_iter(b'+', b'=', b'$', self.grid))
            .chain(memchr3_iter(b'%', b'/', b'&', self.grid))
            .chain(memchr_iter(b'-', self.grid))
            .map(|index| {
                Position::new(
                    (index % (self.width + 1)).try_into().unwrap(),
                    (index / (self.width + 1)).try_into().unwrap(),
                )
            })
    }

    fn stars(&self) -> impl Iterator<Item = Position> + '_ {
        memchr_iter(b'*', self.grid).map(|index| {
            Position::new(
                (index % (self.width + 1)).try_into().unwrap(),
                (index / (self.width + 1)).try_into().unwrap(),
            )
        })
    }
}

pub fn day3_part1(input: &[u8]) -> u32 {
    let grid = Grid::new(input);
    grid.symbols()
        .map(|p| numbers_for_symbol(&grid, p).iter().sum::<u32>())
        .sum()
}

pub fn day3_part2(input: &[u8]) -> u32 {
    let grid = Grid::new(input);
    grid.stars()
        .filter_map(|position| {
            let numbers = numbers_for_symbol(&grid, position);
            if numbers.len() == 2 {
                Some(numbers[0] * numbers[1])
            } else {
                None
            }
        })
        .sum()
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

fn numbers_for_symbol(input: &Grid<'_>, position: Position) -> ArrayVec<u32, 6> {
    let mut array = ArrayVec::new();

    if let b'0'..=b'9' = *input.get(position.left()) {
        let start = find_number_start(input, position.left());
        let (value, _) = parse_number(input.slice_after(start));
        array.push(value);
    }

    if let b'0'..=b'9' = *input.get(position.right()) {
        let (value, _) = parse_number(input.slice_after(position.right()));
        array.push(value);
    }

    match *input.get(position.up()) {
        b'0'..=b'9' => {
            let start = find_number_start(input, position.up());
            let (value, _) = parse_number(input.slice_after(start));
            array.push(value);
        }
        _ => {
            let topleft = position.up().left();
            if let b'0'..=b'9' = *input.get(topleft) {
                let start = find_number_start(input, topleft);
                let (value, _) = parse_number(input.slice_after(start));
                array.push(value);
            }
            let topright = position.up().right();
            if let b'0'..=b'9' = *input.get(topright) {
                let (value, _) = parse_number(input.slice_after(topright));
                array.push(value);
            }
        }
    }

    match *input.get(position.down()) {
        b'0'..=b'9' => {
            let start = find_number_start(input, position.down());
            let (value, _) = parse_number(input.slice_after(start));
            array.push(value);
        }
        _ => {
            let bottomleft = position.down().left();
            if let b'0'..=b'9' = *input.get(bottomleft) {
                let start = find_number_start(input, bottomleft);
                let (value, _) = parse_number(input.slice_after(start));
                array.push(value);
            }
            let bottomright = position.down().right();
            if let b'0'..=b'9' = *input.get(bottomright) {
                let (value, _) = parse_number(input.slice_after(bottomright));
                array.push(value);
            }
        }
    }

    array
}

fn find_number_start(grid: &Grid<'_>, mut position: Position) -> Position {
    loop {
        let new = position.left();
        if let b'0'..=b'9' = *grid.get(new) {
            position = new;
        } else {
            break position;
        }
    }
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

    #[test]
    fn finds_number_at_top_left_of_symbol() {
        let grid = Grid::new(b"123.\n...*\n");
        let position = grid.symbols().next().unwrap();
        assert_eq!(position, Position::new(3, 1));
        assert_eq!(numbers_for_symbol(&grid, position), {
            let mut v = ArrayVec::new();
            v.push(123);
            v
        });
    }

    #[test]
    fn finds_number_at_top_center_of_symbol() {
        let grid = Grid::new(b"123.\n..*.\n");
        let position = grid.symbols().next().unwrap();
        assert_eq!(position, Position::new(2, 1));
        assert_eq!(numbers_for_symbol(&grid, position), {
            let mut v = ArrayVec::new();
            v.push(123);
            v
        });
        let grid = Grid::new(b"123\n.*.\n");
        let position = grid.symbols().next().unwrap();
        assert_eq!(position, Position::new(1, 1));
        assert_eq!(numbers_for_symbol(&grid, position), {
            let mut v = ArrayVec::new();
            v.push(123);
            v
        });
        let grid = Grid::new(b"123\n*..\n");
        let position = grid.symbols().next().unwrap();
        assert_eq!(position, Position::new(0, 1));
        assert_eq!(numbers_for_symbol(&grid, position), {
            let mut v = ArrayVec::new();
            v.push(123);
            v
        });
    }

    #[test]
    fn finds_number_at_top_right_of_symbol() {
        let grid = Grid::new(b".123\n*...\n");
        let position = grid.symbols().next().unwrap();
        assert_eq!(position, Position::new(0, 1));
        assert_eq!(numbers_for_symbol(&grid, position), {
            let mut v = ArrayVec::new();
            v.push(123);
            v
        });
    }

    #[test]
    fn finds_number_left_of_symbol() {
        let grid = Grid::new(b"123*\n");
        let position = grid.symbols().next().unwrap();
        assert_eq!(position, Position::new(3, 0));
        assert_eq!(numbers_for_symbol(&grid, position), {
            let mut v = ArrayVec::new();
            v.push(123);
            v
        });
    }

    #[test]
    fn finds_number_right_of_symbol() {
        let grid = Grid::new(b"*123\n");
        let position = grid.symbols().next().unwrap();
        assert_eq!(position, Position::new(0, 0));
        assert_eq!(numbers_for_symbol(&grid, position), {
            let mut v = ArrayVec::new();
            v.push(123);
            v
        });
    }

    #[test]
    fn finds_number_at_bottom_left_of_symbol() {
        let grid = Grid::new(b"...*\n123*\n");
        let position = grid.symbols().next().unwrap();
        assert_eq!(position, Position::new(3, 0));
        assert_eq!(numbers_for_symbol(&grid, position), {
            let mut v = ArrayVec::new();
            v.push(123);
            v
        });
    }

    #[test]
    fn finds_number_at_bottom_center_of_symbol() {
        let grid = Grid::new(b"..*.\n123.\n");
        let position = grid.symbols().next().unwrap();
        assert_eq!(position, Position::new(2, 0));
        assert_eq!(numbers_for_symbol(&grid, position), {
            let mut v = ArrayVec::new();
            v.push(123);
            v
        });
        let grid = Grid::new(b".*.\n123\n");
        let position = grid.symbols().next().unwrap();
        assert_eq!(position, Position::new(1, 0));
        assert_eq!(numbers_for_symbol(&grid, position), {
            let mut v = ArrayVec::new();
            v.push(123);
            v
        });
        let grid = Grid::new(b"*..\n123\n");
        let position = grid.symbols().next().unwrap();
        assert_eq!(position, Position::new(0, 0));
        assert_eq!(numbers_for_symbol(&grid, position), {
            let mut v = ArrayVec::new();
            v.push(123);
            v
        });
    }

    #[test]
    fn finds_number_at_bottom_right_of_symbol() {
        let grid = Grid::new(b"*...\n.123\n");
        let position = grid.symbols().next().unwrap();
        assert_eq!(position, Position::new(0, 0));
        assert_eq!(numbers_for_symbol(&grid, position), {
            let mut v = ArrayVec::new();
            v.push(123);
            v
        });
    }

    #[test]
    fn test_day3_part1_example() {
        let input = utils::load_example(3);
        assert_eq!(day3_part1(&input), 4361);
    }

    #[test]
    fn test_day3_part2_example() {
        let input = utils::load_example(3);
        assert_eq!(day3_part2(&input), 467835);
    }

    #[test]
    fn test_day3_part1_real() {
        let input = utils::load_real(3);
        assert_eq!(day3_part1(&input), 539637);
    }

    #[test]
    fn test_day3_part2_real() {
        let input = utils::load_real(3);
        assert_eq!(day3_part2(&input), 82818007);
    }
}
