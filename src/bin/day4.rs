use std::{ops::RangeInclusive, str::FromStr};

fn main() {
    let assignments: Vec<_> = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<ElfAssignmentPair>())
        .map(Result::unwrap)
        .collect();

    dbg!(
        "part1",
        assignments
            .iter()
            .filter(|a| a.has_fully_redundant_assignment())
            .count()
    );

    dbg!(
        "part2",
        assignments.iter().filter(|a| a.has_redundancy()).count()
    );
}

#[derive(Debug, PartialEq, Eq)]
struct ElfAssignmentPair(ElfAssignment, ElfAssignment);

impl ElfAssignmentPair {
    fn has_fully_redundant_assignment(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn has_redundancy(&self) -> bool {
        self.0.intersect(&self.1).is_some()
    }
}

impl FromStr for ElfAssignmentPair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (e1, e2) = s.split_once(',').ok_or(ParseError)?;

        Ok(ElfAssignmentPair(e1.parse()?, e2.parse()?))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ElfAssignment(RangeInclusive<SectionId>);

impl ElfAssignment {
    fn contains(&self, other: &Self) -> bool {
        self.0.contains(other.0.start()) && self.0.contains(other.0.end())
    }

    fn intersect(&self, other: &Self) -> Option<ElfAssignment> {
        let start = SectionId::max(*self.0.start(), *other.0.start());
        let end = SectionId::min(*self.0.end(), *other.0.end());

        if end >= start {
            Some(ElfAssignment(start..=end))
        } else {
            None
        }
    }
}

impl FromStr for ElfAssignment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(ParseError)?;

        let start = start.parse().map_err(|_| ParseError)?;
        let end = end.parse().map_err(|_| ParseError)?;

        Ok(ElfAssignment(start..=end))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct SectionId(u64);

impl FromStr for SectionId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SectionId(s.parse::<u64>()?))
    }
}

#[derive(Debug)]
struct ParseError;
