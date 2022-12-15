use std::{collections::HashSet, ops::RangeInclusive};

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|l| !l.is_empty());

    let mut map = input.flat_map(parse_path).collect::<HashSet<_>>();

    let floor_y = map.iter().map(|(_x, y)| y).max().unwrap() + 1;

    let mut part1_done = false;

    for i in 1.. {
        let mut grain_pos = (500, 0);
        while let Some(next) = grain_next_pos(&map, grain_pos) {
            if grain_pos.1 >= floor_y && !part1_done {
                println!("{}", i - 1);
                part1_done = true;
            }

            if grain_pos.1 == floor_y {
                break;
            }

            grain_pos = next;
        }

        if grain_pos == (500, 0) {
            println!("{i}");
            break;
        }

        map.insert(grain_pos);
    }
}

fn grain_next_pos(map: &HashSet<(usize, usize)>, (x, y): (usize, usize)) -> Option<(usize, usize)> {
    if !map.contains(&(x, y + 1)) {
        Some((x, y + 1))
    } else if !map.contains(&(x.saturating_sub(1), y + 1)) {
        Some((x - 1, y + 1))
    } else if !map.contains(&(x + 1, y + 1)) {
        Some((x + 1, y + 1))
    } else {
        None
    }
}

fn parse_path(input: String) -> impl Iterator<Item = (usize, usize)> {
    let mut input = input.split(" -> ").map(parse_coord);

    #[derive(Debug)]
    enum Path {
        Horiz(RangeInclusive<usize>, usize),
        Vert(usize, RangeInclusive<usize>),
    }

    impl Iterator for Path {
        type Item = (usize, usize);

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Path::Horiz(x_it, y) => {
                    let x = x_it.next()?;
                    Some((x, *y))
                }
                Path::Vert(x, y_it) => {
                    let y = y_it.next()?;
                    Some((*x, y))
                }
            }
        }
    }

    let mut paths = Vec::with_capacity(input.size_hint().0.max(2) - 1);
    let mut prev = input.next().unwrap();
    for next in input {
        let path = if prev.0 == next.0 {
            let min = prev.1.min(next.1);
            let max = prev.1.max(next.1);
            Path::Vert(prev.0, min..=max)
        } else if prev.1 == next.1 {
            let min = prev.0.min(next.0);
            let max = prev.0.max(next.0);
            Path::Horiz(min..=max, prev.1)
        } else {
            unreachable!()
        };

        paths.push(path);
        prev = next;
    }

    paths.into_iter().flatten()
}

fn parse_coord(input: &str) -> (usize, usize) {
    let (x, y) = input.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

#[derive(Debug)]
struct ParseError;
