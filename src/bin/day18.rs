use std::{collections::HashSet, io::stdin, ops::Add, str::FromStr};

fn main() {
    let puzzle = Puzzle::parse(
        stdin()
            .lines()
            .map(Result::unwrap)
            .filter(|s| !s.is_empty()),
    );

    // part 1
    println!("{}", puzzle.solve1());

    // part 2
    println!("{}", puzzle.solve2());
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord3(i64, i64, i64);

#[derive(Debug)]
struct Puzzle(HashSet<Coord3>);

impl Puzzle {
    fn parse(input: impl Iterator<Item = String>) -> Self {
        let cubes = input.map(|s| s.parse::<Coord3>().unwrap()).collect();
        Puzzle(cubes)
    }

    fn solve1(&self) -> usize {
        self.0
            .iter()
            .map(|&c| self.count_exposed_sides(c))
            .sum::<usize>()
    }

    fn solve2(&self) -> usize {
        let bounding_box = self.bounding_box();
        let point_outside_min = bounding_box.0 + Coord3(-1, -1, -1);
        let point_outside_max = bounding_box.1 + Coord3(1, 1, 1);
        let bounding_box = bounding_box
            .extend(point_outside_max)
            .extend(point_outside_min);

        let mut visited = HashSet::with_capacity(self.0.len() * 6);
        let mut queue = Vec::new();
        queue.push((point_outside_min, point_outside_max));

        let mut count = 0;
        while let Some((comes_from, point)) = queue.pop() {
            if !bounding_box.contains(point) || visited.contains(&(comes_from, point)) {
                continue;
            }

            visited.insert((comes_from, point));

            if self.0.contains(&point) {
                count += 1;
            } else {
                for neigh in point.neighbors().filter(|&n| n != comes_from) {
                    queue.push((point, neigh));
                }
            }
        }

        count
    }

    fn count_exposed_sides(&self, cube_pos: Coord3) -> usize {
        cube_pos
            .neighbors()
            .filter(|coord| !self.0.contains(coord))
            .count()
    }

    fn bounding_box(&self) -> Coord3Range {
        self.0.iter().copied().fold(
            Coord3Range::new(self.0.iter().copied().next().unwrap()),
            Coord3Range::extend,
        )
    }
}

#[derive(Debug)]
struct Coord3Range(Coord3, Coord3);

impl Coord3Range {
    fn new(coord: Coord3) -> Self {
        Coord3Range(coord, coord)
    }

    fn extend(self, coord: Coord3) -> Self {
        let Coord3Range(mut min, mut max) = self;
        min.0 = min.0.min(coord.0);
        min.1 = min.1.min(coord.1);
        min.2 = min.2.min(coord.1);
        max.0 = max.0.max(coord.0);
        max.1 = max.1.max(coord.1);
        max.2 = max.2.max(coord.2);

        Coord3Range(min, max)
    }

    fn contains(&self, coord: Coord3) -> bool {
        let Coord3Range(min, max) = &self;
        coord.0 >= min.0
            && coord.0 <= max.0
            && coord.1 >= min.1
            && coord.1 <= max.1
            && coord.2 >= min.2
            && coord.2 <= max.2
    }
}

impl Coord3 {
    fn neighbors(self) -> impl Iterator<Item = Self> {
        [
            Coord3(0, 0, 1),
            Coord3(0, 0, -1),
            Coord3(0, 1, 0),
            Coord3(0, -1, 0),
            Coord3(1, 0, 0),
            Coord3(-1, 0, 0),
        ]
        .into_iter()
        .map(move |delta| self + delta)
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
