use std::str::FromStr;

fn main() {
    let rucksacks: Vec<_> = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Rucksack>())
        .map(Result::unwrap)
        .collect();

    dbg!(
        "part1",
        rucksacks
            .iter()
            .map(|r| r.duplicate_priority_sum())
            .sum::<u64>()
    );

    let badge_priorities = rucksacks
        .chunks_exact(3)
        .map(|group| {
            group
                .iter()
                .map(|x| x.all_items())
                .reduce(|x, y| x.intersection(y))
                .expect("invalid group")
        })
        .map(|set| {
            assert_eq!(set.len(), 1);
            set.iter_priorities().next().unwrap()
        });
    dbg!("part2", badge_priorities.sum::<u64>());
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Rucksack(ElfItemSet, ElfItemSet);

impl Rucksack {
    fn duplicate_priorities(&self) -> impl Iterator<Item = u64> {
        self.0.intersection(self.1).iter_priorities()
    }

    fn duplicate_priority_sum(&self) -> u64 {
        self.duplicate_priorities().sum()
    }

    fn all_items(&self) -> ElfItemSet {
        self.0.union(self.1)
    }
}

impl FromStr for Rucksack {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            return Err(ParseError);
        }

        let (c1, c2) = s.split_at(s.len() / 2);
        debug_assert_eq!(c1.len(), c2.len());

        Ok(Rucksack(c1.parse()?, c2.parse()?))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct ElfItemSet(u64);

impl ElfItemSet {
    fn mark_item(&mut self, i: usize) {
        self.0 |= 1 << i;
    }

    fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    fn len(self) -> u32 {
        self.0.count_ones()
    }

    fn iter_priorities(self) -> impl Iterator<Item = u64> {
        RucksackCompartmentSetPriorityIter {
            set: self.0,
            idx: 0,
        }
    }
}

impl FromStr for ElfItemSet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut compartment_items = ElfItemSet(0);

        for b in s.bytes() {
            let idx = if (b'a'..=b'z').contains(&b) {
                b - b'a'
            } else if (b'A'..=b'Z').contains(&b) {
                b - b'A' + 26
            } else {
                return Err(ParseError);
            };

            compartment_items.mark_item(idx as usize);
        }

        Ok(compartment_items)
    }
}

struct RucksackCompartmentSetPriorityIter {
    set: u64,
    idx: u64,
}

impl Iterator for RucksackCompartmentSetPriorityIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while (self.set & (1 << self.idx)) == 0 && self.idx < 26 * 2 {
            self.idx += 1
        }
        self.idx += 1;

        if self.idx <= 26 * 2 {
            Some(self.idx)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct ParseError;
