fn main() {
    let input = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|l| !l.is_empty());

    let map = TreeHeightMap::parse(input);

    dbg!(
        "part1",
        map.compute_visibility().flatten().filter(|v| *v).count()
    );
}

struct TreeHeightMap(Vec<Vec<i8>>);

impl TreeHeightMap {
    fn parse(input: impl Iterator<Item = String>) -> Self {
        let map = input
            .map(|line| line.bytes().map(|c| (c - b'0') as i8).collect())
            .collect();

        TreeHeightMap(map)
    }

    fn compute_visibility(&self) -> impl Iterator<Item = impl Iterator<Item = bool> + '_> {
        self.compute_lr_visibility()
            .zip(self.compute_rl_visibility())
            .zip(self.compute_bt_visibility())
            .zip(self.compute_tb_visibility())
            .map(|(((it1, it2), it3), it4)| {
                it1.zip(it2)
                    .zip(it3)
                    .zip(it4)
                    .map(|(((v1, v2), v3), v4)| v1 || v2 || v3 || v4)
            })
    }

    fn compute_lr_visibility(&self) -> impl Iterator<Item = impl Iterator<Item = bool> + '_> {
        self.0.iter().map(|line| {
            line.iter().scan(-1, |state, &tree_height| {
                let res = tree_height > *state;
                *state = (*state).max(tree_height);
                Some(res)
            })
        })
    }

    fn compute_rl_visibility(&self) -> impl Iterator<Item = impl Iterator<Item = bool> + '_> {
        self.0.iter().map(|line| {
            line.iter()
                .rev()
                .scan(-1, |state, &tree_height| {
                    let res = tree_height > *state;
                    *state = (*state).max(tree_height);
                    Some(res)
                })
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
        })
    }

    fn compute_tb_visibility(&self) -> impl Iterator<Item = impl Iterator<Item = bool> + '_> {
        self.0
            .iter()
            .scan(vec![-1; self.0[0].len()], |state, tree_heights| {
                let res = state
                    .clone()
                    .into_iter()
                    .zip(tree_heights.iter())
                    .map(|(max_h, h)| *h > max_h);

                state.iter_mut().zip(tree_heights).for_each(|(h, h2)| {
                    *h = (*h).max(*h2);
                });
                Some(res)
            })
    }

    fn compute_bt_visibility(&self) -> impl Iterator<Item = impl Iterator<Item = bool> + '_> {
        self.0
            .iter()
            .rev()
            .scan(vec![-1; self.0[0].len()], |state, tree_heights| {
                let res = state
                    .clone()
                    .into_iter()
                    .zip(tree_heights.iter())
                    .map(|(max_h, h)| *h > max_h);

                state.iter_mut().zip(tree_heights).for_each(|(h, h2)| {
                    *h = (*h).max(*h2);
                });
                Some(res)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
    }
}
