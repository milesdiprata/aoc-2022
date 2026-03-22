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
#[repr(usize)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    rows: usize,
    cols: usize,
    bounds_x: Vec<Range<usize>>,
    bounds_y: Vec<Range<usize>>,
}

#[derive(Debug)]
struct Path {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Face {
    grid: Pos<usize>,
    origin: Pos<usize>,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    idx: usize,
    dir: Direction,
    coord_flipped: bool,
}

#[derive(Debug)]
struct Cube {
    board: Board,
    face_size: usize,
    faces: [Face; Self::FACE_LEN],
    edges: [[Edge; Direction::LEN]; Self::FACE_LEN],
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
        for y in 0..self.rows {
            if y > 0 {
                f.write_char('\n')?;
            }

            for x in 0..self.cols {
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

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.board.rows {
            if y > 0 {
                f.write_char('\n')?;
            }

            for x in 0..self.board.cols {
                if self.board.get(Pos::new(x, y)).is_some() {
                    let grid = Pos::new(x / self.face_size, y / self.face_size);
                    let face_num = self
                        .faces
                        .iter()
                        .position(|face| face.grid == grid)
                        .ok_or(std::fmt::Error)?
                        + 1;
                    write!(f, "{face_num}")?;
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
            rows,
            cols,
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

impl TryFrom<Board> for Cube {
    type Error = Error;

    fn try_from(board: Board) -> Result<Self> {
        let face_size = Self::compute_face_size(&board);
        dbg!(face_size);
        let faces = Self::compute_faces(&board, face_size)?;
        dbg!(&faces);
        let edges = Self::compute_adj_edges(&faces);
        dbg!(&edges);
        let edges = Self::compute_fold_edges(edges)?;
        dbg!(&edges);

        Ok(Self {
            board,
            face_size,
            faces,
            edges,
        })
    }
}

impl Tile {
    const fn is_open(self) -> bool {
        matches!(self, Self::Open)
    }
}

impl Direction {
    const LEN: usize = 4;

    const fn turn(self, turn: Orientation) -> Self {
        match (self, turn) {
            (Self::Right, Orientation::Left) | (Self::Left, Orientation::Right) => Self::Up,
            (Self::Right, Orientation::Right) | (Self::Left, Orientation::Left) => Self::Down,
            (Self::Up, Orientation::Right) | (Self::Down, Orientation::Left) => Self::Right,
            (Self::Up, Orientation::Left) | (Self::Down, Orientation::Right) => Self::Left,
        }
    }

    const fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
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

    fn step(&self, pos: Pos<usize>, dir: Direction) -> Option<Pos<usize>> {
        let next = match dir {
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
        };

        self.get(next).is_some_and(Tile::is_open).then_some(next)
    }
}

impl Cube {
    const FACE_LEN: usize = 6;

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss
    )]
    fn compute_face_size(board: &Board) -> usize {
        let tiles = board.tiles.iter().map(Vec::len).sum::<usize>();
        ((tiles / 6) as f64).sqrt() as usize
    }

    fn compute_faces(board: &Board, face_size: usize) -> Result<[Face; Self::FACE_LEN]> {
        let height = board.rows / face_size;
        let width = board.cols / face_size;

        let mut faces = Vec::with_capacity(6);

        for y in 0..height {
            for x in 0..width {
                let origin = Pos::new(x * face_size, y * face_size);
                if board.get(origin).is_some() {
                    faces.push(Face {
                        grid: Pos::new(x, y),
                        origin,
                    });
                }
            }
        }

        faces
            .try_into()
            .map_err(|faces: Vec<_>| faces.len())
            .map_err(|len| anyhow!("expected 6 faces but found {len}"))
    }

    fn compute_adj_edges(
        faces: &[Face; Self::FACE_LEN],
    ) -> [[Option<Edge>; Direction::LEN]; Self::FACE_LEN] {
        let mut edges = [[None; Direction::LEN]; Self::FACE_LEN];

        for (i, a) in faces.iter().enumerate() {
            for (j, b) in faces.iter().enumerate() {
                if i == j {
                    continue;
                }

                // B is directly to the right of A in grid
                if a.grid.y() == b.grid.y()
                    && a.grid.right().is_some_and(|right| right.x() == b.grid.x())
                {
                    edges[i][Direction::Right as usize] = Some(Edge {
                        idx: j,
                        dir: Direction::Right,
                        coord_flipped: false,
                    });
                    edges[j][Direction::Left as usize] = Some(Edge {
                        idx: i,
                        dir: Direction::Left,
                        coord_flipped: false,
                    });
                }

                // B is directly below A in grid
                if a.grid.x() == b.grid.x()
                    && a.grid.down().is_some_and(|down| down.y() == b.grid.y())
                {
                    edges[i][Direction::Down as usize] = Some(Edge {
                        idx: j,
                        dir: Direction::Down,
                        coord_flipped: false,
                    });
                    edges[j][Direction::Up as usize] = Some(Edge {
                        idx: i,
                        dir: Direction::Up,
                        coord_flipped: false,
                    });
                }
            }
        }

        edges
    }

    fn compute_fold_edges(
        mut edges: [[Option<Edge>; Direction::LEN]; Self::FACE_LEN],
    ) -> Result<[[Edge; Direction::LEN]; Self::FACE_LEN]> {
        fn propagate(edges: &mut [[Option<Edge>; Direction::LEN]; Cube::FACE_LEN]) -> bool {
            let mut changed = false;
            for f in 0..6 {
                for d1 in [
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                ] {
                    let d2 = d1.turn(Orientation::Right); // adjacent clockwise direction

                    let Some((
                        Edge {
                            idx: idx1,
                            dir: enter1,
                            coord_flipped: flip1,
                        },
                        Edge {
                            idx: idx2,
                            dir: enter2,
                            coord_flipped: flip2,
                        },
                    )) = edges[f][d1 as usize].zip(edges[f][d2 as usize])
                    else {
                        continue;
                    };

                    // On face N1, the edge "clockwise from e1" should connect to N2
                    // On face N2, the edge "counter-clockwise from e2" should connect to N1
                    let idx1_edge = if flip1 {
                        enter1.turn(Orientation::Left)
                    } else {
                        enter1.turn(Orientation::Right)
                    };
                    let idx2_edge = if flip2 {
                        enter2.turn(Orientation::Right)
                    } else {
                        enter2.turn(Orientation::Left)
                    };

                    let rev = flip1 != flip2; // XOR — reversal propagates

                    if edges[idx1][idx1_edge as usize].is_none() {
                        edges[idx1][idx1_edge as usize] = Some(Edge {
                            idx: idx2,
                            dir: idx2_edge.opposite(),
                            coord_flipped: rev,
                        });
                        changed = true;
                    }

                    if edges[idx2][idx2_edge as usize].is_none() {
                        edges[idx2][idx2_edge as usize] = Some(Edge {
                            idx: idx1,
                            dir: idx1_edge.opposite(),
                            coord_flipped: rev,
                        });
                        changed = true;
                    }
                }
            }

            changed
        }

        while propagate(&mut edges) {}

        edges
            .into_iter()
            .map(|edges| {
                edges
                    .map(|e| e.ok_or_else(|| anyhow!("missing edge")))
                    .into_iter()
                    .collect::<Result<Vec<_>>>()?
                    .try_into()
                    .map_err(|_| anyhow!("unexpected edge count"))
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|_| anyhow!("unexpected face count"))
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
                    if let Some(next) = board.step(pos, dir) {
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

fn part2(cube: &Cube, path: &Path) -> u64 {
    println!("{cube}");
    todo!()
}

fn main() -> Result<()> {
    let (board, path) = self::parse()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&board, &path);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        // assert_eq!(part1, 73_346);
    };

    {
        let cube = Cube::try_from(board)?;
        println!("{cube}");
        let start = Instant::now();
        let part2 = self::part2(&cube, &path);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        // assert_eq!(part2, 0);
    };

    Ok(())
}
