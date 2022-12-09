use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
    str::FromStr,
};

fn main() {
    let movements = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Movement>().unwrap());

    let mut rope = Rope::default();

    let mut visited_tail_positions = HashSet::new();
    for mov in movements {
        let step_tail_positions = rope.translate_head_track_tail(mov);
        visited_tail_positions.extend(step_tail_positions);
    }

    println!("{}", visited_tail_positions.len());
}

#[derive(Default)]
struct Rope {
    head_coord: Vector,
    tail_coord: Vector,
}

impl Rope {
    fn translate_head_track_tail(&mut self, mut mov: Movement) -> Vec<Vector> {
        let mut tail_positions = Vec::with_capacity(mov.amount as usize);

        while let Some(new_head) = mov.translate_coord_step(self.head_coord) {
            self.head_coord = new_head;

            let tail_update = match self.head_coord - self.tail_coord {
                Vector(x, y) if x.abs() <= 1 && y.abs() <= 1 => Vector(0, 0),

                Vector(x, 0) => Vector(x.signum(), 0),
                Vector(0, y) => Vector(0, y.signum()),

                Vector(x, y) => Vector(x.signum(), y.signum()),
            };
            self.tail_coord += tail_update;

            tail_positions.push(self.tail_coord);
        }

        tail_positions
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Movement {
    direction: Direction,
    amount: u64,
}

impl Movement {
    fn translate_coord_step(&mut self, coord: Vector) -> Option<Vector> {
        if self.amount == 0 {
            None
        } else {
            let dir: Vector = self.direction.into();
            self.amount -= 1;
            Some(coord + dir)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<Vector> for Direction {
    fn into(self) -> Vector {
        use Direction::*;
        match self {
            Up => Vector(0, 1),
            Down => Vector(0, -1),
            Left => Vector(-1, 0),
            Right => Vector(1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Vector(i64, i64);

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        let Vector(x1, y1) = self;
        let Vector(x2, y2) = rhs;

        Vector(x1 + x2, y1 + y2)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        let Vector(x, y) = self;
        Vector(-x, -y)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.neg())
    }
}

impl Mul<i64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i64) -> Self::Output {
        let Vector(x, y) = self;
        Vector(x * rhs, y * rhs)
    }
}

impl Div<i64> for Vector {
    type Output = Vector;

    fn div(self, rhs: i64) -> Self::Output {
        let Vector(x, y) = self;
        Vector(x / rhs, y / rhs)
    }
}

impl FromStr for Movement {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s.split_once(' ').ok_or(ParseErr)?;

        let direction = direction.parse().map_err(|_| ParseErr)?;

        let amount = amount.parse().map_err(|_| ParseErr)?;

        Ok(Movement { direction, amount })
    }
}

impl FromStr for Direction {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;

        Ok(match s {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => return Err(ParseErr),
        })
    }
}

#[derive(Debug, Clone)]
struct ParseErr;
