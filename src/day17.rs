use std::collections::VecDeque;
use std::fs;
use std::ops::Range;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::bail;

#[derive(Clone, Copy, Debug)]
enum Jet {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Row {
    bits: u8,
}

#[derive(Clone, Copy, Debug)]
struct Rock {
    height: usize,
    rows: [Row; 4],
}

#[derive(Debug)]
struct Chamber {
    rows: VecDeque<Row>,
}

impl std::fmt::Display for Jet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
        }
    }
}

impl std::fmt::Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self
            .rows()
            .map(Row::bit_range)
            .map(|range| range.start)
            .min()
            .unwrap_or_default();
        let x_max = self
            .rows()
            .map(Row::bit_range)
            .map(|range| range.end)
            .max()
            .unwrap_or_default();

        for (i, row) in self.rows().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }

            for x in (x_min..x_max).rev() {
                write!(f, "{}", if row.bits & (1 << x) != 0 { '#' } else { '.' })?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &row) in self.rows.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }

            write!(f, "|")?;
            for x in (0..Self::WIDTH).rev() {
                let c = if row.bits & (1 << x) != 0 { '#' } else { '.' };
                write!(f, "{c}")?;
            }
            write!(f, "|")?;
        }

        writeln!(f)?;
        write!(f, "+{}+", "-".repeat(Self::WIDTH as usize))?;

        Ok(())
    }
}

impl TryFrom<char> for Jet {
    type Error = Error;

    fn try_from(jet: char) -> Result<Self> {
        match jet {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => bail!("invalid jet '{jet}'"),
        }
    }
}

impl Row {
    const fn from(bits: u8) -> Self {
        Self { bits }
    }

    const fn shift(self, jet: Jet) -> Option<Self> {
        const WALL: u8 = !((1 << Chamber::WIDTH) - 1);

        let shifted = match jet {
            Jet::Left => self.bits << 1,
            Jet::Right => self.bits >> 1,
        };

        let reverted = match jet {
            Jet::Left => shifted >> 1,
            Jet::Right => shifted << 1,
        };

        if reverted == self.bits && shifted & WALL == 0 {
            Some(Self::from(shifted))
        } else {
            None
        }
    }

    const fn is_empty(self) -> bool {
        self.bits == 0
    }

    const fn collides(self, other: Self) -> bool {
        self.bits & other.bits != 0
    }

    const fn merge(mut self, other: Self) -> Self {
        self.bits |= other.bits;
        self
    }

    fn bit_range(self) -> Range<u8> {
        let mut x_min = 8_u8;
        let mut x_max = 0_u8;

        for b in 0..8 {
            if self.bits & (1 << b) != 0 {
                x_min = x_min.min(b);
                x_max = x_max.max(b + 1);
            }
        }

        x_min..x_max
    }
}

#[allow(clippy::unreadable_literal)]
impl Rock {
    const VARIANTS: [Self; 5] = [
        Self::horizontal(),
        Self::plus(),
        Self::l_shape(),
        Self::vertical(),
        Self::square(),
    ];

    const fn horizontal() -> Self {
        Self {
            height: 1,
            rows: [
                Row::from(0b00011110),
                Row::from(0b00000000),
                Row::from(0b00000000),
                Row::from(0b00000000),
            ],
        }
    }

    const fn plus() -> Self {
        Self {
            height: 3,
            rows: [
                Row::from(0b00001000),
                Row::from(0b00011100),
                Row::from(0b00001000),
                Row::from(0b00000000),
            ],
        }
    }

    const fn l_shape() -> Self {
        Self {
            height: 3,
            rows: [
                Row::from(0b00000100),
                Row::from(0b00000100),
                Row::from(0b00011100),
                Row::from(0b00000000),
            ],
        }
    }

    const fn vertical() -> Self {
        Self {
            height: 4,
            rows: [
                Row::from(0b00010000),
                Row::from(0b00010000),
                Row::from(0b00010000),
                Row::from(0b00010000),
            ],
        }
    }

    const fn square() -> Self {
        Self {
            height: 2,
            rows: [
                Row::from(0b00011000),
                Row::from(0b00011000),
                Row::from(0b00000000),
                Row::from(0b00000000),
            ],
        }
    }

    fn shift(self, jet: Jet) -> Option<Self> {
        let mut shifted = self;

        for i in 0..self.height {
            shifted.rows[i] = self.rows[i].shift(jet)?;
        }

        Some(shifted)
    }

    fn rows(self) -> impl Iterator<Item = Row> {
        self.rows.into_iter().take(self.height)
    }
}

impl Chamber {
    const WIDTH: u8 = 7;

    const fn new() -> Self {
        Self {
            rows: VecDeque::new(),
        }
    }

    #[allow(dead_code)]
    fn display_with_rock(&self, rock: Rock, y: usize) -> String {
        use std::fmt::Write;

        let mut out = String::new();

        for (i, &row) in self.rows.iter().enumerate() {
            if i > 0 {
                writeln!(out).unwrap();
            }

            let rock_bits = if i >= y && i < y + rock.height {
                rock.rows[i - y].bits
            } else {
                0
            };

            write!(out, "|").unwrap();
            for x in (0..Self::WIDTH).rev() {
                let mask = 1 << x;
                let c = if rock_bits & mask != 0 {
                    '@'
                } else if row.bits & mask != 0 {
                    '#'
                } else {
                    '.'
                };
                write!(out, "{c}").unwrap();
            }
            write!(out, "|").unwrap();
        }

        writeln!(out).unwrap();
        write!(out, "+{}+", "-".repeat(Self::WIDTH as usize)).unwrap();

        out
    }

    fn collides(&self, rock: Rock, y: usize) -> bool {
        for (i, row_rock) in rock.rows().enumerate() {
            let Some(&row) = self.rows.get(y + i) else {
                return true;
            };

            if row.collides(row_rock) {
                return true;
            }
        }

        false
    }

    fn settle(&mut self, rock: Rock, y: usize) {
        for (i, row_rock) in rock.rows().enumerate() {
            self.rows[y + i] = self.rows[y + i].merge(row_rock);
        }
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn drop_rock(&mut self, mut rock: Rock, jets: &[Jet], jet: &mut usize) {
        let gaps = 3 + rock.height;
        for _ in 0..gaps {
            self.rows.push_front(Row::from(0));
        }

        let mut y = 0;

        // println!("Rock begins falling:");
        // println!("{}\n", self.display_with_rock(rock, y));

        loop {
            let jet_dir = jets[*jet];
            if let Some(shifted) = rock.shift(jet_dir)
                && !self.collides(shifted, y)
            {
                // println!("Jet of gas pushes rock {jet_dir}:");
                rock = shifted;
            } else {
                // println!("Jet of gas pushes rock {jet_dir}, but nothing happens:");
            }
            *jet = (*jet + 1) % jets.len();
            // println!("{}\n", self.display_with_rock(rock, y));

            if self.collides(rock, y + 1) {
                self.settle(rock, y);
                // println!("Rock comes to rest:");
                // println!("{self}\n");
                break;
            }
            y += 1;
            // println!("Rock falls 1 unit:");
            // println!("{}\n", self.display_with_rock(rock, y));
        }

        while self.rows.front().copied().is_some_and(Row::is_empty) {
            self.rows.pop_front();
        }
    }
}

fn part1(jets: &[Jet]) -> usize {
    const ROCKS: usize = 2022;

    let mut chamber = Chamber::new();
    let mut jet = 0;

    for rock in Rock::VARIANTS.into_iter().cycle().take(ROCKS) {
        chamber.drop_rock(rock, jets, &mut jet);
    }

    chamber.height()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let jets = fs::read_to_string("in/day17.txt")?
        .chars()
        .map(Jet::try_from)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&jets);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 3_232);
    };

    {
        let start = Instant::now();
        let part2 = self::part2();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        // assert_eq!(part2, 0);
    };

    Ok(())
}
