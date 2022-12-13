use std::{cmp::Ordering, io::stdin};

use nom::{branch, bytes::complete::tag, combinator, multi, sequence, IResult};

fn main() {
    let mut packets = stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|s| !s.is_empty())
        .map(|s| parse_packet(&s).unwrap().1)
        .collect::<Vec<_>>();

    /*
    // part1
    println!(
        "{}",
        packets
            .chunks_exact(2)
            .enumerate()
            .filter(|(_, chunks)| (chunks[0] <= chunks[1]))
            .map(|(pair_idx, _)| pair_idx + 1)
            .sum::<usize>()
    );
    */

    let div2: Packet = Packet::divider::<2>();
    let div6: Packet = Packet::divider::<6>();

    packets.push(div2.clone());
    packets.push(div6.clone());
    packets.sort_unstable();

    let div2_idx = packets.iter().position(|p| *p == div2).unwrap() + 1;
    let div6_idx = packets.iter().position(|p| *p == div6).unwrap() + 1;

    println!("{}", div2_idx * div6_idx);
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Integer(i64),
    List(Vec<Packet>),
}

impl Packet {
    fn divider<const N: i64>() -> Self {
        Packet::List(vec![Packet::List(vec![Packet::Integer(N)])])
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    branch::alt((
        combinator::map(nom::character::complete::i64, Packet::Integer),
        combinator::map(
            sequence::delimited(
                tag("["),
                multi::separated_list0(tag(","), parse_packet),
                tag("]"),
            ),
            Packet::List,
        ),
    ))(input)
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Integer(i1), Integer(i2)) => i1.cmp(i2),
            (Integer(i1), p2) => List(vec![Integer(*i1)]).cmp(p2),
            (p1, Integer(i2)) => p1.cmp(&List(vec![Integer(*i2)])),
            (List(l1), List(l2)) => compare_packet_list(l1, l2),
        }
    }
}

fn compare_packet_list(l1: &[Packet], l2: &[Packet]) -> Ordering {
    match (l1, l2) {
        ([], []) => Ordering::Equal,
        ([], _) => Ordering::Less,
        (_, []) => Ordering::Greater,
        ([p1, rest1 @ ..], [p2, rest2 @ ..]) => p1.cmp(p2).then(rest1.cmp(rest2)),
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
