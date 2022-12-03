use std::str::FromStr;

fn main() {
    let rucksacks = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Rucksack>())
        .map(Result::unwrap);

    dbg!(rucksacks.map(|r| r.duplicate_priority_sum()).sum::<u64>());
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Rucksack(RucksackCompartmentSet, RucksackCompartmentSet);

impl Rucksack {
    fn duplicate_priorities(&self) -> impl Iterator<Item = u64> {
        self.0.intersection(self.1).iter_priorities()
    }

    fn duplicate_priority_sum(&self) -> u64 {
        self.duplicate_priorities().sum()
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
struct RucksackCompartmentSet(u64);

impl RucksackCompartmentSet {
    fn mark_item(&mut self, i: usize) {
        self.0 |= 1 << i;
    }

    fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    fn iter_priorities(self) -> impl Iterator<Item = u64> {
        RucksackCompartmentSetPriorityIter {
            set: self.0,
            idx: 0,
        }
    }
}

impl FromStr for RucksackCompartmentSet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut compartment_items = RucksackCompartmentSet(0);

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
