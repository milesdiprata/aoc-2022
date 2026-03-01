use std::fmt::Write;
use std::fs;
use std::ops::Range;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

use aoc_2022::Pos;

#[derive(Clone, Copy, Debug)]
enum Tile {
    Open,
    Wall,
}

#[derive(Clone, Copy, Debug)]
enum Orientation {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Move(usize),
    Turn(Orientation),
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    bounds_x: Vec<Range<usize>>,
    bounds_y: Vec<Range<usize>>,
}

#[derive(Debug)]
struct Path {
    instructions: Vec<Instruction>,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => f.write_char('.'),
            Self::Wall => f.write_char('#'),
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = self.bounds_x.len();
        let cols = self
            .bounds_x
            .iter()
            .map(|range| range.end)
            .max()
            .ok_or(std::fmt::Error)?;

        for y in 0..rows {
            if y > 0 {
                f.write_char('\n')?;
            }

            for x in 0..cols {
                if let Some(tile) = self.get(Pos::new(x, y)) {
                    write!(f, "{tile}")?;
                } else {
                    f.write_char(' ')?;
                }
            }
        }

        Ok(())
    }
}

impl From<Direction> for u8 {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = Error;

    fn try_from(tile: u8) -> Result<Self> {
        match tile {
            b'.' => Ok(Self::Open),
            b'#' => Ok(Self::Wall),
            _ => bail!("invalid tile {tile}"),
        }
    }
}

impl TryFrom<char> for Orientation {
    type Error = Error;

    fn try_from(orientation: char) -> Result<Self> {
        match orientation {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => bail!("invalid orientation {orientation}"),
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(instruction: &str) -> Result<Self> {
        if let Ok(steps) = instruction.parse() {
            Ok(Self::Move(steps))
        } else if let Ok(orientation) = Orientation::try_from(
            instruction
                .chars()
                .next()
                .ok_or_else(|| anyhow!("invalid instruction '{instruction}'"))?,
        ) {
            Ok(Self::Turn(orientation))
        } else {
            bail!("invalid instruction '{instruction}'");
        }
    }
}

impl FromStr for Board {
    type Err = Error;

    fn from_str(board: &str) -> Result<Self> {
        let rows = board.lines().count();
        let cols = board
            .lines()
            .map(str::len)
            .max()
            .ok_or_else(|| anyhow!("invalid board"))?;

        let mut tiles = vec![Vec::with_capacity(cols); rows];
        let mut bounds_x = Vec::with_capacity(rows);

        for (i, row) in board.lines().map(str::as_bytes).enumerate() {
            let mut start = 0;
            let mut end = 0;

            while end < row.len() {
                match Tile::try_from(row[end]) {
                    Ok(tile) => tiles[i].push(tile),
                    Err(_) => {
                        if tiles[i].is_empty() {
                            start += 1;
                        } else {
                            break;
                        }
                    }
                }

                end += 1;
            }

            bounds_x.push(start..end);
        }

        let bounds_y = (0..cols)
            .map(|x| {
                let start = bounds_x
                    .iter()
                    .position(|range| range.contains(&x))
                    .unwrap();
                let end = bounds_x
                    .iter()
                    .rposition(|range| range.contains(&x))
                    .unwrap()
                    + 1;
                start..end
            })
            .collect();

        Ok(Self {
            tiles,
            bounds_x,
            bounds_y,
        })
    }
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(path: &str) -> Result<Self> {
        let mut instructions = Vec::with_capacity(path.len());
        let mut i = 0;
        let bytes = path.as_bytes();

        while let Some(&byte) = bytes.get(i) {
            if byte.is_ascii_digit() {
                let start = i;
                while bytes.get(i).is_some_and(u8::is_ascii_digit) {
                    i += 1;
                }
                instructions.push(path[start..i].parse()?);
            } else {
                instructions.push(path[i..=i].parse()?);
                i += 1;
            }
        }

        Ok(Self { instructions })
    }
}

impl Tile {
    const fn is_open(self) -> bool {
        matches!(self, Self::Open)
    }
}

impl Direction {
    const fn turn(self, turn: Orientation) -> Self {
        match (self, turn) {
            (Self::Right, Orientation::Left) | (Self::Left, Orientation::Right) => Self::Up,
            (Self::Right, Orientation::Right) | (Self::Left, Orientation::Left) => Self::Down,
            (Self::Up, Orientation::Right) | (Self::Down, Orientation::Left) => Self::Right,
            (Self::Up, Orientation::Left) | (Self::Down, Orientation::Right) => Self::Left,
        }
    }
}

impl Board {
    fn get(&self, pos: Pos<usize>) -> Option<Tile> {
        if let Some(range) = self.bounds_x.get(pos.y())
            && range.contains(&pos.x())
        {
            Some(self.tiles[pos.y()][pos.x() - range.start])
        } else {
            None
        }
    }

    fn start(&self) -> Option<(Pos<usize>, Direction)> {
        self.bounds_x
            .iter()
            .enumerate()
            .flat_map(|(y, range)| range.clone().map(move |x| Pos::new(x, y)))
            .find(|&pos| self.get(pos).is_some_and(Tile::is_open))
            .map(|pos| (pos, Direction::Right))
    }

    fn step(&self, pos: Pos<usize>, dir: Direction) -> Pos<usize> {
        match dir {
            Direction::Right => match pos.right() {
                Some(right) if self.bounds_x[pos.y()].contains(&right.x()) => right,
                _ => Pos::new(self.bounds_x[pos.y()].start, pos.y()),
            },
            Direction::Left => match pos.left() {
                Some(left) if self.bounds_x[pos.y()].contains(&left.x()) => left,
                _ => Pos::new(self.bounds_x[pos.y()].end - 1, pos.y()),
            },
            Direction::Down => match pos.down() {
                Some(down) if self.bounds_y[pos.x()].contains(&down.y()) => down,
                _ => Pos::new(pos.x(), self.bounds_y[pos.x()].start),
            },
            Direction::Up => match pos.up() {
                Some(up) if self.bounds_y[pos.x()].contains(&up.y()) => up,
                _ => Pos::new(pos.x(), self.bounds_y[pos.x()].end - 1),
            },
        }
    }
}

fn parse() -> Result<(Board, Path)> {
    let input = fs::read_to_string("in/day22.txt")?;
    let (board, path) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("invalid input"))?;

    Ok((Board::from_str(board)?, Path::from_str(path)?))
}

fn part1(board: &Board, path: &Path) -> usize {
    let (mut pos, mut dir) = board.start().unwrap();

    for &instr in &path.instructions {
        match instr {
            Instruction::Move(steps) => {
                for _ in 0..steps {
                    let next = board.step(pos, dir);
                    if board.get(next).is_some_and(Tile::is_open) {
                        pos = next;
                    } else {
                        break;
                    }
                }
            }
            Instruction::Turn(turn) => dir = dir.turn(turn),
        }
    }

    (4 * (pos.x() + 1)) + (1_000 * (pos.y() + 1)) + usize::from(u8::from(dir))
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let (board, path) = self::parse()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&board, &path);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 73_346);
    };

    {
        let start = Instant::now();
        let part2 = self::part2();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 0);
    };

    Ok(())
}
