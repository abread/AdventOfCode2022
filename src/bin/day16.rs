use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    hash::Hash,
    io::stdin,
};

use nom::{branch, bytes::complete::tag, combinator, multi, sequence, IResult};

fn main() {
    let valve_info = stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|s| !s.is_empty())
        .map(|s| parse_valve_info(&s).unwrap().1)
        .collect::<HashMap<_, _>>();

    println!("{}", weighted_bfs_dist(&valve_info, 30));
}

fn weighted_bfs_dist(vi: &HashMap<ValveId, ValveInfo>, time: usize) -> i64 {
    #[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Clone)]
    struct State {
        time_left: usize,
        current_node: ValveId,
        open_nodes: BTreeMap<ValveId, usize>,
        pressure_acc: i64,
    }

    let initial_state = State {
        time_left: time,
        current_node: ['A', 'A'],
        open_nodes: BTreeMap::new(),
        pressure_acc: 0,
    };

    let max_valve_flow = vi.values().map(|info| info.flow_rate).max().unwrap();

    let mut open_set = PriorityQueue::new();
    open_set.push(
        initial_state,
        max_valve_flow * (time / 2 * (time / 2 + 1)) as i64,
    );

    while let Some((cur_st, _cur_dist)) = open_set.pop() {
        if cur_st.time_left == 0 {
            // reached goal
            return cur_st.pressure_acc;
        }

        let next_states = if !cur_st.open_nodes.contains_key(&cur_st.current_node)
            && vi[&cur_st.current_node].flow_rate > 0
        {
            std::iter::once(Some(State {
                time_left: cur_st.time_left - 1,
                current_node: cur_st.current_node,
                open_nodes: {
                    let mut on = cur_st.open_nodes.clone();
                    on.insert(cur_st.current_node, cur_st.time_left);
                    on
                },
                pressure_acc: cur_st.pressure_acc
                    + (cur_st.time_left - 1) as i64 * vi[&cur_st.current_node].flow_rate,
            }))
        } else {
            std::iter::once(None)
        }
        .flatten()
        .chain(
            vi[&cur_st.current_node]
                .path_to
                .iter()
                .map(|&neigh_idx| State {
                    time_left: cur_st.time_left - 1,
                    current_node: neigh_idx,
                    open_nodes: cur_st.open_nodes.clone(),
                    pressure_acc: cur_st.pressure_acc,
                }),
        );

        for neigh_st in next_states {
            let time_left = neigh_st.time_left as i64;
            let generous_possible_rem_flow_est =
                max_valve_flow * ((time_left / 2) * (time_left / 2 + 1)) / 2;
            let prio = -(neigh_st.pressure_acc + generous_possible_rem_flow_est);
            open_set.push(neigh_st, prio);
        }
    }

    unreachable!();
}

#[derive(Debug, Clone)]
struct ValveInfo {
    flow_rate: i64,
    path_to: Vec<ValveId>,
}

fn parse_valve_info(input: &str) -> IResult<&str, (ValveId, ValveInfo)> {
    combinator::map(
        sequence::tuple((
            sequence::preceded(tag("Valve "), parse_valve_id),
            sequence::preceded(tag(" has flow rate="), nom::character::complete::i64),
            sequence::preceded(
                branch::alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                multi::separated_list1(tag(", "), parse_valve_id),
            ),
        )),
        |(id, flow_rate, path_to)| (id, ValveInfo { flow_rate, path_to }),
    )(input)
}

type ValveId = [char; 2];

fn parse_valve_id(input: &str) -> IResult<&str, ValveId> {
    combinator::map(nom::bytes::complete::take(2_usize), |s: &str| {
        let mut it = s.chars();
        [it.next().unwrap(), it.next().unwrap()]
    })(input)
}

#[derive(Default)]
struct PriorityQueue<T, Prio>(BTreeMap<Prio, BTreeSet<T>>);

impl<T: Ord + Eq + Clone, Prio: Ord + Clone> PriorityQueue<T, Prio> {
    fn new() -> Self {
        PriorityQueue(BTreeMap::new())
    }

    fn push(&mut self, item: T, prio: Prio) {
        self.0.entry(prio).or_default().insert(item);
    }

    fn pop(&mut self) -> Option<(T, Prio)> {
        loop {
            let prio = self.0.keys().next()?.clone();

            if let Some(x) = self.pop_prio(&prio) {
                break Some((x, prio));
            }
        }
    }

    fn pop_prio(&mut self, prio: &Prio) -> Option<T> {
        let queue = self.0.get_mut(prio).unwrap();

        match queue.iter().next() {
            Some(x) => {
                let x = x.clone();
                queue.remove(&x);
                Some(x)
            }
            None => {
                self.0.remove(prio);
                None
            }
        }
    }
}
