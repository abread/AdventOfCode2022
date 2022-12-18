use std::{
    collections::HashSet,
    io::{stdin, Read},
    ops::Range,
};

fn main() {
    // part 1
    const N_ROCKS: usize = 2022;
    // part 2
    // const N_ROCKS: usize = 1000000000000;
    const X_LIMITS: Range<i64> = 0..7;

    let rocks: [Vec<(i64, i64)>; 5] = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let mut rock_map = HashSet::with_capacity(
        (N_ROCKS / rocks.len() + 1) * rocks.iter().map(|r| r.len()).sum::<usize>(),
    );

    let jet_pattern = stdin()
        .bytes()
        .filter_map(|b| {
            let b = b.unwrap();
            match b {
                b'>' => Some((1, 0)),
                b'<' => Some((-1, 0)),
                _ => None,
            }
        })
        .collect::<Vec<_>>();
    let mut jet_pattern = jet_pattern.into_iter().cycle();

    for rock_shape in (rocks.iter()).cycle().take(N_ROCKS) {
        let mut next_pos = (
            2,
            rock_map.iter().map(|&(_x, y)| y).max().unwrap_or(-1) + 1 + 3,
        );

        loop {
            let jet_transl = jet_pattern.next().unwrap();
            if !has_overlap(
                &rock_map,
                X_LIMITS,
                vec_sum(next_pos, jet_transl),
                rock_shape,
            ) {
                next_pos = vec_sum(next_pos, jet_transl);
            }

            if has_overlap(&rock_map, X_LIMITS, vec_sum(next_pos, (0, -1)), rock_shape) {
                let rock_points = rock_shape.iter().map(|&p| vec_sum(p, next_pos));
                rock_map.extend(rock_points);
                break;
            } else {
                next_pos = vec_sum(next_pos, (0, -1));
            }
        }
    }

    println!(
        "{}",
        rock_map.iter().map(|&(_x, y)| y).max().unwrap_or(-1) + 1
    );
}

fn vec_sum((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> (i64, i64) {
    (x1 + x2, y1 + y2)
}

fn has_overlap(
    occupied_positions: &HashSet<(i64, i64)>,
    x_limits: Range<i64>,
    p0: (i64, i64),
    rock_shape: &[(i64, i64)],
) -> bool {
    for &rock_point in rock_shape {
        let rock_point = vec_sum(rock_point, p0);

        if rock_point.1 < 0
            || !x_limits.contains(&rock_point.0)
            || occupied_positions.contains(&rock_point)
        {
            return true;
        }
    }

    false
}
