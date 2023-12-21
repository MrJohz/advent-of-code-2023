use std::fmt::Debug;

pub fn day10_part1(input: &[u8]) -> i32 {
    0
}

pub fn day10_part2(input: &[u8]) -> i32 {
    0
}

fn parse_grid(input: &[u8]) -> Grid {
    let mut width = 0;
    let mut start = Position { x: 0, y: 0 };
    let mut x = 0;
    let mut y = 0;
    let data = input
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            let pipe = match c {
                b'|' => Pipes::Vertical,
                b'-' => Pipes::Horizontal,
                b'L' => Pipes::NorthAndEast,
                b'J' => Pipes::NorthAndWest,
                b'F' => Pipes::SouthAndEast,
                b'7' => Pipes::SouthAndWest,
                b'.' => Pipes::Empty,
                b'S' => {
                    start = Position { x, y };
                    Pipes::StartingPosition
                }
                b'\n' => {
                    x = 0;
                    y += 1;
                    if width == 0 {
                        width = i;
                    }
                    return None;
                }
                _ => panic!("Invalid character at {}: {}", i, c),
            };
            x += 1;
            Some(pipe)
        })
        .collect();
    Grid {
        data,
        start,
        width,
        height: y as usize,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
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

#[derive(Debug, Clone, Copy)]
enum Pipes {
    Vertical,
    Horizontal,
    NorthAndEast,
    NorthAndWest,
    SouthAndEast,
    SouthAndWest,
    Empty,
    StartingPosition,
}

struct Grid {
    data: Vec<Pipes>,
    start: Position,
    width: usize,
    height: usize,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in self.data.chunks(self.width) {
            for pipe in row {
                match pipe {
                    Pipes::Vertical => write!(f, "║")?,
                    Pipes::Horizontal => write!(f, "═")?,
                    Pipes::NorthAndEast => write!(f, "╚")?,
                    Pipes::NorthAndWest => write!(f, "╝")?,
                    Pipes::SouthAndEast => write!(f, "╔")?,
                    Pipes::SouthAndWest => write!(f, "╗")?,
                    Pipes::Empty => write!(f, ".")?,
                    Pipes::StartingPosition => write!(f, "╳")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn get(&self, position: Position) -> Pipes {
        let index = position.x + position.y * self.width as isize;
        if index < 0 || index >= self.data.len() as isize {
            Pipes::Empty
        } else {
            self.data[index as usize]
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::utils;

    use test_case::test_case;

    #[test]
    fn parses_grid_into_valid_shape_1_simple() {
        let input = utils::load_example_with_suffix(10, "1_simple");
        let grid = parse_grid(&input);
        dbg!(&grid);
        assert_eq!(grid.height, 5);
        assert_eq!(grid.width, 5);
        assert_eq!(grid.start, Position { x: 1, y: 1 });
    }

    #[test]
    fn parses_grid_into_valid_shape_1_complex() {
        let input = utils::load_example_with_suffix(10, "1_complex");
        let grid = parse_grid(&input);
        dbg!(&grid);
        assert_eq!(grid.height, 5);
        assert_eq!(grid.width, 5);
        assert_eq!(grid.start, Position { x: 1, y: 1 });
    }

    #[test]
    fn parses_grid_into_valid_shape_2_complex() {
        let input = utils::load_example_with_suffix(10, "2_complex");
        let grid = parse_grid(&input);
        dbg!(&grid);
        assert_eq!(grid.height, 5);
        assert_eq!(grid.width, 5);
        assert_eq!(grid.start, Position { x: 0, y: 2 });
    }

    // #[test]
    // fn test_day10_part1_example() {
    //     let input = utils::load_example(10);
    //     assert_eq!(day10_part1(&input), 114);
    // }

    // #[test]
    // fn test_day10_part2_example() {
    //     let input = utils::load_example(10);
    //     assert_eq!(day10_part2(&input), 2);
    // }

    // #[test]
    // fn test_day10_part1_real() {
    //     let input = utils::load_real(10);
    //     assert_eq!(day10_part1(&input), 1);
    // }

    // #[test]
    // fn test_day10_part2_real() {
    //     let input = utils::load_real(10);
    //     assert_eq!(day10_part2(&input), 1068);
    // }
}
