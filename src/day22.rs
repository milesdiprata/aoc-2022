use std::collections::HashMap;
use std::collections::VecDeque;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Vec3([i32; 3]);

const PX: Vec3 = Vec3([1, 0, 0]);
const PY: Vec3 = Vec3([0, 1, 0]);
const PZ: Vec3 = Vec3([0, 0, 1]);

impl Vec3 {
    const fn cross(self, other: Self) -> Self {
        Self([
            self.0[1] * other.0[2] - self.0[2] * other.0[1],
            self.0[2] * other.0[0] - self.0[0] * other.0[2],
            self.0[0] * other.0[1] - self.0[1] * other.0[0],
        ])
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self([-self.0[0], -self.0[1], -self.0[2]])
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

#[derive(Clone, Copy, Debug)]
struct FaceOrientation {
    normal: Vec3,
    right: Vec3,
    down: Vec3,
}

impl FaceOrientation {
    fn fold(self, grid_dir: Direction) -> Self {
        let o = match grid_dir {
            Direction::Right => Self {
                normal: self.right,
                right: -self.normal,
                down: self.down,
            },
            Direction::Left => Self {
                normal: -self.right,
                right: self.normal,
                down: self.down,
            },
            Direction::Down => Self {
                normal: self.down,
                right: self.right,
                down: -self.normal,
            },
            Direction::Up => Self {
                normal: -self.down,
                right: self.right,
                down: self.normal,
            },
        };

        debug_assert_eq!(o.normal, o.right.cross(o.down));
        o
    }

    fn tangent(self, dir: Direction) -> Vec3 {
        match dir {
            Direction::Up => -self.down,
            Direction::Down => self.down,
            Direction::Left => -self.right,
            Direction::Right => self.right,
        }
    }

    const fn along_edge(self, dir: Direction) -> Vec3 {
        match dir {
            Direction::Up | Direction::Down => self.right,
            Direction::Left | Direction::Right => self.down,
        }
    }
}

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
        let faces = Self::compute_faces(&board, face_size)?;
        let orientations = Self::compute_orientations(&faces)?;
        let edges = Self::compute_edges_from_3d(&orientations)?;

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

    fn compute_orientations(
        faces: &[Face; Self::FACE_LEN],
    ) -> Result<[FaceOrientation; Self::FACE_LEN]> {
        let mut orientations: [Option<FaceOrientation>; Self::FACE_LEN] = [None; Self::FACE_LEN];
        orientations[0] = Some(FaceOrientation {
            normal: PZ,
            right: PX,
            down: PY,
        });

        let mut queue = VecDeque::new();
        queue.push_back(0usize);

        let grid_neighbor = |grid: Pos<usize>, dir: Direction| match dir {
            Direction::Up => grid.up(),
            Direction::Right => grid.right(),
            Direction::Down => grid.down(),
            Direction::Left => grid.left(),
        };

        while let Some(a) = queue.pop_front() {
            let orient_a = orientations[a].unwrap();
            let grid_a = faces[a].grid;

            for dir in [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ] {
                let Some(neighbor) = grid_neighbor(grid_a, dir) else {
                    continue;
                };

                if let Some(b) = faces.iter().position(|f| f.grid == neighbor)
                    && orientations[b].is_none()
                {
                    orientations[b] = Some(orient_a.fold(dir));
                    queue.push_back(b);
                }
            }
        }

        let result: Vec<FaceOrientation> = orientations
            .into_iter()
            .map(|o| o.ok_or_else(|| anyhow!("disconnected face")))
            .collect::<Result<_>>()?;

        debug_assert_eq!(
            result
                .iter()
                .map(|o| o.normal)
                .collect::<std::collections::HashSet<_>>()
                .len(),
            6,
            "all 6 normals must be distinct"
        );

        result
            .try_into()
            .map_err(|_| anyhow!("unexpected face count"))
    }

    fn compute_edges_from_3d(
        orientations: &[FaceOrientation; Self::FACE_LEN],
    ) -> Result<[[Edge; Direction::LEN]; Self::FACE_LEN]> {
        let all_dirs = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        let mut edge_map: HashMap<Vec3, Vec<(usize, Direction)>> = HashMap::new();
        for (f, orient) in orientations.iter().enumerate() {
            for &d in &all_dirs {
                let edge_mid = orient.normal + orient.tangent(d);
                edge_map.entry(edge_mid).or_default().push((f, d));
            }
        }

        let mut edges = [[None::<Edge>; Direction::LEN]; Self::FACE_LEN];

        for entries in edge_map.values() {
            if entries.len() != 2 {
                bail!("expected exactly 2 faces per edge, found {}", entries.len());
            }

            let (f1, d1) = entries[0];
            let (f2, d2) = entries[1];

            let along1 = orientations[f1].along_edge(d1);
            let along2 = orientations[f2].along_edge(d2);
            let coord_flipped = along1 == -along2;

            edges[f1][d1 as usize] = Some(Edge {
                idx: f2,
                dir: d2.opposite(),
                coord_flipped,
            });
            edges[f2][d2 as usize] = Some(Edge {
                idx: f1,
                dir: d1.opposite(),
                coord_flipped,
            });
        }

        edges
            .into_iter()
            .map(|face_edges| {
                face_edges
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

    fn step(
        &self,
        pos: Pos<usize>,
        dir: Direction,
        face_idx: usize,
    ) -> Option<(Pos<usize>, Direction, usize)> {
        let face = &self.faces[face_idx];
        let lx = pos.x() - face.origin.x();
        let ly = pos.y() - face.origin.y();
        let s = self.face_size;

        let (nlx, nly, new_dir, new_face) = match dir {
            Direction::Right if lx + 1 < s => (lx + 1, ly, dir, face_idx),
            Direction::Left if lx > 0 => (lx - 1, ly, dir, face_idx),
            Direction::Down if ly + 1 < s => (lx, ly + 1, dir, face_idx),
            Direction::Up if ly > 0 => (lx, ly - 1, dir, face_idx),
            _ => {
                let edge = &self.edges[face_idx][dir as usize];
                let c = match dir {
                    Direction::Up | Direction::Down => lx,
                    Direction::Left | Direction::Right => ly,
                };
                let c = if edge.coord_flipped { s - 1 - c } else { c };
                let new_dir = edge.dir;
                let (nlx, nly) = match new_dir {
                    Direction::Right => (0, c),
                    Direction::Left => (s - 1, c),
                    Direction::Down => (c, 0),
                    Direction::Up => (c, s - 1),
                };
                (nlx, nly, new_dir, edge.idx)
            }
        };

        let new_pos = Pos::new(
            self.faces[new_face].origin.x() + nlx,
            self.faces[new_face].origin.y() + nly,
        );

        self.board
            .get(new_pos)
            .is_some_and(Tile::is_open)
            .then_some((new_pos, new_dir, new_face))
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

fn part2(cube: &Cube, path: &Path) -> usize {
    let (mut pos, mut dir) = cube.board.start().unwrap();
    let mut face_idx = cube
        .faces
        .iter()
        .position(|f| {
            pos.x() >= f.origin.x()
                && pos.x() < f.origin.x() + cube.face_size
                && pos.y() >= f.origin.y()
                && pos.y() < f.origin.y() + cube.face_size
        })
        .unwrap();

    for &instr in &path.instructions {
        match instr {
            Instruction::Move(steps) => {
                for _ in 0..steps {
                    if let Some((new_pos, new_dir, new_face)) = cube.step(pos, dir, face_idx) {
                        pos = new_pos;
                        dir = new_dir;
                        face_idx = new_face;
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
        let start = Instant::now();
        let part2 = self::part2(&cube, &path);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        // assert_eq!(part2, 0);
    };

    Ok(())
}
