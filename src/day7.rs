use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Debug)]
enum FileKind {
    Dir { children: Vec<usize> },
    Regular { size: usize },
}

#[derive(Debug)]
struct File {
    name: String,
    kind: FileKind,
    parent: Option<usize>,
}

#[derive(Debug)]
struct FileSystem {
    files: Vec<File>,
}

impl std::fmt::Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stack = vec![(&self.files[0], 0_usize)];

        while let Some((file, level)) = stack.pop() {
            if file.name != "/" {
                writeln!(f)?;
            }

            for _ in 0..level {
                write!(f, "  ")?;
            }

            write!(f, "- ")?;

            match &file.kind {
                FileKind::Dir { children } => {
                    write!(f, "{} (dir)", &file.name)?;

                    let mut sorted: Vec<_> = children.clone();
                    sorted.sort_by(|&a, &b| self.files[a].name.cmp(&self.files[b].name));

                    for &child in sorted.iter().rev() {
                        stack.push((&self.files[child], level + 1));
                    }
                }
                &FileKind::Regular { size } => write!(f, "{} (file, size={size})", &file.name)?,
            }
        }

        Ok(())
    }
}

impl FromStr for FileSystem {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut fs = Self::new();
        let mut cwd = 0;

        for block in input
            .split('$')
            .map(str::trim)
            .filter(|&block| !block.is_empty())
        {
            if let Some(dir) = block.strip_prefix("cd ") {
                cwd = match dir {
                    "/" => 0,
                    ".." => fs.files[cwd].parent.unwrap(),
                    name => match &fs.files[cwd].kind {
                        FileKind::Dir { children } => children
                            .iter()
                            .copied()
                            .find(|&c| fs.files[c].name == name)
                            .unwrap_or_else(|| {
                                fs.add(name.to_string(), FileKind::Dir { children: vec![] }, cwd)
                            }),
                        FileKind::Regular { .. } => bail!("cd into non-directory '{dir}'"),
                    },
                };
            } else if block.starts_with("ls") {
                for line in block.lines().skip(1) {
                    if let Some(name) = line.strip_prefix("dir ") {
                        fs.add(name.to_string(), FileKind::Dir { children: vec![] }, cwd);
                    } else {
                        let (size, name) = line
                            .split_once(' ')
                            .ok_or_else(|| anyhow!("invalid file '{line}'"))?;
                        fs.add(
                            name.to_string(),
                            FileKind::Regular {
                                size: size.parse()?,
                            },
                            cwd,
                        );
                    }
                }
            } else {
                bail!("unknown command in block '{block}'")
            }
        }

        Ok(fs)
    }
}

impl File {
    const fn is_dir(&self) -> bool {
        matches!(self.kind, FileKind::Dir { .. })
    }
}

impl FileSystem {
    fn new() -> Self {
        Self {
            files: vec![File {
                name: "/".to_string(),
                kind: FileKind::Dir { children: vec![] },
                parent: None,
            }],
        }
    }

    fn add(&mut self, name: String, kind: FileKind, parent: usize) -> usize {
        let idx = self.files.len();

        self.files.push(File {
            name,
            kind,
            parent: Some(parent),
        });

        if let FileKind::Dir { children } = &mut self.files[parent].kind {
            children.push(idx);
        }

        idx
    }

    fn size(&self, file: usize) -> usize {
        match &self.files[file].kind {
            FileKind::Dir { children } => children.iter().map(|&c| self.size(c)).sum(),
            &FileKind::Regular { size } => size,
        }
    }
}

fn part1(fs: &FileSystem) -> usize {
    const SIZE_MAX: usize = 100_000;

    (0..fs.files.len())
        .filter(|&file| fs.files[file].is_dir())
        .map(|file| fs.size(file))
        .filter(|&size| size <= SIZE_MAX)
        .sum()
}

fn part2(fs: &FileSystem) -> usize {
    const SIZE_DISK: usize = 70_000_000;
    const SIZE_UNUSED_DESIRED: usize = 30_000_000;

    let used = fs.size(0);
    let unused = SIZE_DISK - used;

    let mut files = (0..fs.files.len())
        .filter(|&file| fs.files[file].is_dir())
        .map(|file| Reverse(fs.size(file)))
        .collect::<BinaryHeap<_>>();

    while let Some(Reverse(size)) = files.pop() {
        if unused + size >= SIZE_UNUSED_DESIRED {
            return size;
        }
    }

    used
}

fn main() -> Result<()> {
    let fs = FileSystem::from_str(&fs::read_to_string("in/day7.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&fs);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_334_506);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&fs);
        let elapsed = Instant::now().duration_since(start);

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 7_421_137);
    };

    Ok(())
}
