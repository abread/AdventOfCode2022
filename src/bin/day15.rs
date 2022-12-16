use std::{collections::HashSet, io::stdin, ops::Range};

use nom::{bytes::complete::tag, combinator, sequence, IResult};

fn main() {
    let sensor_info = stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|s| !s.is_empty())
        .map(|s| parse_sensor_info(&s).unwrap().1)
        .collect::<Vec<_>>();

    let areas: Vec<CoveredArea> = sensor_info
        .iter()
        .cloned()
        .map(|info| info.into())
        .collect();

    // dbg!(solve_part1(&sensor_info, &areas, 10));
    // println!("{}", solve_part1(&sensor_info, &areas, 2000000));

    let limit = 0..(4000000 + 1);
    //dbg!(solve_part2(&sensor_info, &areas, (0..21, 0..21)));
    println!(
        "{}",
        solve_part2(&sensor_info, &areas, (limit.clone(), limit))
    );
}

fn solve_part1(sensor_info: &[SensorInfo], areas: &[CoveredArea], target_y: i64) -> usize {
    let mut occupied_x_ranges = areas
        .iter()
        .map(|area| area.xx_at_y(target_y))
        .filter(|r| !r.is_empty())
        .collect::<HashSet<_>>();

    dedupe_range_set(&mut occupied_x_ranges);

    occupied_x_ranges
        .into_iter()
        .map(|r| r.count())
        .sum::<usize>()
        - sensor_info
            .iter()
            .map(|info| info.beacon_pos)
            .filter(|&(_, y)| y == target_y)
            .collect::<HashSet<_>>()
            .len()
}

fn find_beacon(
    _sensor_info: &[SensorInfo],
    areas: &[CoveredArea],
    limits: (Range<i64>, Range<i64>),
) -> (i64, i64) {
    for y in limits.1.clone() {
        let min_x = limits.0.start;
        let max_x = limits.0.end;

        let mut occupied_x_ranges = areas
            .iter()
            .map(|area| area.xx_at_y(y))
            .filter(|r| !r.is_empty())
            .map(move |r| r.start.max(min_x)..r.end.min(max_x))
            .filter(|r| !r.is_empty())
            .collect::<HashSet<_>>();

        dedupe_range_set(&mut occupied_x_ranges);

        for r in occupied_x_ranges {
            if r == limits.0 {
                break;
            } else if r.start > limits.0.start {
                return (r.start - 1, y);
            } else if r.end < limits.0.end {
                return (r.end, y);
            } else {
                unreachable!();
            }
        }
    }

    unreachable!();
}

fn solve_part2(
    sensor_info: &[SensorInfo],
    areas: &[CoveredArea],
    limits: (Range<i64>, Range<i64>),
) -> i64 {
    let beacon_pos = dbg!(find_beacon(sensor_info, areas, limits));

    beacon_pos.0 * 4000000 + beacon_pos.1
}

fn dedupe_range_set(range_set: &mut HashSet<Range<i64>>) {
    while let Some((r1, r2)) = range_set
        .iter()
        .enumerate()
        .flat_map(|(i, range)| std::iter::repeat(range).zip(range_set.iter().skip(i + 1)))
        .find(|(r1, r2)| {
            r1.contains(&r2.start)
                || r1.contains(&r2.end)
                || r2.contains(&r1.start)
                || r2.contains(&r1.end)
        })
    {
        let r1 = r1.clone();
        let r2 = r2.clone();

        range_set.remove(&r1);
        range_set.remove(&r2);

        range_set.insert(r1.start.min(r2.start)..r1.end.max(r2.end));
    }
}

#[derive(Debug)]
struct CoveredArea {
    center: (i64, i64),
    radius: i64,
}

impl CoveredArea {
    fn contains(&self, pos: (i64, i64)) -> bool {
        manhattan_distance(self.center, pos) <= self.radius
    }

    fn edges(&self) -> (i64, i64, i64, i64) {
        (
            self.center.0 - self.radius,
            self.center.1 - self.radius,
            self.center.0 + self.radius,
            self.center.1 + self.radius,
        )
    }

    fn xx_at_y(&self, y: i64) -> Range<i64> {
        let range = self.radius - (y - self.center.1).abs();

        if range >= 0 {
            self.center.0 - range..(self.center.0 + range + 1)
        } else {
            0..0 // empty
        }
    }
}

impl From<SensorInfo> for CoveredArea {
    fn from(info: SensorInfo) -> Self {
        CoveredArea {
            center: info.sensor_pos,
            radius: manhattan_distance(info.sensor_pos, info.beacon_pos),
        }
    }
}

fn manhattan_distance((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

#[derive(Debug, Clone)]
struct SensorInfo {
    sensor_pos: (i64, i64),
    beacon_pos: (i64, i64),
}

fn parse_sensor_info(input: &str) -> IResult<&str, SensorInfo> {
    combinator::map(
        sequence::separated_pair(
            sequence::preceded(tag("Sensor at "), parse_coord),
            tag(": "),
            sequence::preceded(tag("closest beacon is at "), parse_coord),
        ),
        |(sensor_pos, beacon_pos)| SensorInfo {
            sensor_pos,
            beacon_pos,
        },
    )(input)
}

fn parse_coord(input: &str) -> IResult<&str, (i64, i64)> {
    sequence::separated_pair(
        sequence::preceded(tag("x="), nom::character::complete::i64),
        tag(", "),
        sequence::preceded(tag("y="), nom::character::complete::i64),
    )(input)
}
