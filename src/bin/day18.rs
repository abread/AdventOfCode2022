use std::{collections::HashSet, io::stdin, ops::Add, str::FromStr};

fn main() {
    let puzzle = Puzzle::parse(
        stdin()
            .lines()
            .map(Result::unwrap)
            .filter(|s| !s.is_empty()),
    );

    // part 1
    println!(
        "{}",
        puzzle
            .0
            .iter()
            .map(|&c| puzzle.count_exposed_sides(c))
            .sum::<usize>()
    );
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord3(i64, i64, i64);

struct Puzzle(HashSet<Coord3>);

impl Puzzle {
    fn parse(input: impl Iterator<Item = String>) -> Self {
        let cubes = input.map(|s| s.parse::<Coord3>().unwrap()).collect();
        Puzzle(cubes)
    }

    fn count_exposed_sides(&self, cube_pos: Coord3) -> usize {
        [
            Coord3(0, 0, 1),
            Coord3(0, 0, -1),
            Coord3(0, 1, 0),
            Coord3(0, -1, 0),
            Coord3(1, 0, 0),
            Coord3(-1, 0, 0),
        ]
        .into_iter()
        .map(|delta| delta + cube_pos)
        .filter(|coord| !self.0.contains(coord))
        .count()
    }
}

impl FromStr for Coord3 {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().ok_or(ParseErr)?;
        let y = parts.next().ok_or(ParseErr)?;
        let z = parts.next().ok_or(ParseErr)?;

        Ok(Coord3(
            x.parse().map_err(|_| ParseErr)?,
            y.parse().map_err(|_| ParseErr)?,
            z.parse().map_err(|_| ParseErr)?,
        ))
    }
}

impl Add for Coord3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

#[derive(Debug)]
struct ParseErr;
