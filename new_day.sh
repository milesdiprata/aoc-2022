#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi

DAY="$1"
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"

INPUT_FILE="$PROJECT_DIR/in/day${DAY}.txt"
SRC_FILE="$PROJECT_DIR/src/day${DAY}.rs"
CARGO_TOML="$PROJECT_DIR/Cargo.toml"

# Check if files already exist
if [ -f "$INPUT_FILE" ]; then
    echo "Error: $INPUT_FILE already exists"
    exit 1
fi
if [ -f "$SRC_FILE" ]; then
    echo "Error: $SRC_FILE already exists"
    exit 1
fi

# 1. Create blank input file
touch "$INPUT_FILE"
echo "Created $INPUT_FILE"

# 2. Create source file with scaffolding
cat > "$SRC_FILE" << EOF
use std::fs;
use std::time::Instant;

use anyhow::Result;

fn part1() -> u64 {
    todo!()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let _input = fs::read_to_string("in/day${DAY}.txt")?;

    {
        let start = Instant::now();
        let part1 = self::part1();
        let elapsed = Instant::now().duration_since(start);

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 0);
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
EOF
echo "Created $SRC_FILE"

# 3. Add binary entry to Cargo.toml
cat >> "$CARGO_TOML" << EOF

[[bin]]
name = "day${DAY}"
path = "src/day${DAY}.rs"
EOF
echo "Updated $CARGO_TOML"

echo "Done! Scaffolding for day ${DAY} is ready."
