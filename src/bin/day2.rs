fn main() {
    let strategy_guide = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|s| !s.is_empty())
        .map(Round::parse);

    dbg!(strategy_guide.map(Round::score).sum::<u64>());
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Round(Shape, Shape);

impl Round {
    fn parse(line: String) -> Self {
        let (a, b) = line.split_once(' ').unwrap();

        let a = match a {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => unreachable!(),
        };

        let b = match b {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => unreachable!(),
        };

        Round(a, b)
    }

    fn score(self) -> u64 {
        use Shape::*;
        let chosen_shape_score = self.1.score();

        let outcome_score = match self {
            Round(x, y) if x == y => 3,
            Round(Rock, Paper) | Round(Paper, Scissors) | Round(Scissors, Rock) => 6,
            _ => 0,
        };

        chosen_shape_score + outcome_score
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
