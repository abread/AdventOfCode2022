use std::str::FromStr;

fn main() {
    let strategy_guide = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<RoundStrategy>())
        .map(Result::unwrap);

    dbg!(strategy_guide.map(|s| s.score()).sum::<u64>());
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct RoundStrategy(Shape, Outcome);

impl RoundStrategy {
    fn pick_shape(&self) -> Shape {
        use Outcome::*;
        use Shape::*;

        macro_rules! pick_with_rules {
            ($($a1:ident beats $a2:ident),+) => {
                match &self {
                    RoundStrategy(x, Draw) => *x,
                    $(
                        RoundStrategy($a2, Win) => $a1,
                        RoundStrategy($a1, Lose) => $a2,
                    )+
                }
            };
        }

        pick_with_rules! {
            Paper beats Rock,
            Rock beats Scissors,
            Scissors beats Paper
        }
    }

    fn score(&self) -> u64 {
        self.pick_shape().score() + self.1.score()
    }
}

impl FromStr for RoundStrategy {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shape, outcome) = s.split_once(' ').ok_or(ParseError)?;

        Ok(RoundStrategy(shape.parse()?, outcome.parse()?))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(self) -> u64 {
        use Shape::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Shape::*;

        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            _ => Err(ParseError),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(self) -> u64 {
        use Outcome::*;

        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Outcome::*;

        match s {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug)]
struct ParseError;
