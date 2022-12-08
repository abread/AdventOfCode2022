use take_until::TakeUntilExt;

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|l| !l.is_empty())
        .map(|s| &*Box::leak(s.into_boxed_str()));

    let map = TreeHeightMap::parse(input);

    // part 1
    println!(
        "{}",
        map.compute_visibility().flatten().filter(|v| *v).count()
    );

    // part 2
    println!("{}", map.compute_scenic_score().flatten().max().unwrap());
}

struct TreeHeightMap(Vec<Vec<i8>>);

impl TreeHeightMap {
    fn parse<'s>(input: impl Iterator<Item = &'s str>) -> Self {
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
            line.iter().scan(-1, |max_height, &tree_height| {
                let res = tree_height > *max_height;
                *max_height = (*max_height).max(tree_height);
                Some(res)
            })
        })
    }

    fn compute_rl_visibility(&self) -> impl Iterator<Item = impl Iterator<Item = bool> + '_> {
        self.0.iter().map(|line| {
            line.iter()
                .rev()
                .scan(-1, |max_height, &tree_height| {
                    let res = tree_height > *max_height;
                    *max_height = (*max_height).max(tree_height);
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
            .scan(vec![-1; self.0[0].len()], |max_heights, tree_heights| {
                let res = max_heights
                    .clone()
                    .into_iter()
                    .zip(tree_heights.iter())
                    .map(|(max_h, h)| *h > max_h);

                max_heights
                    .iter_mut()
                    .zip(tree_heights)
                    .for_each(|(h, h2)| {
                        *h = (*h).max(*h2);
                    });
                Some(res)
            })
    }

    fn compute_bt_visibility(&self) -> impl Iterator<Item = impl Iterator<Item = bool> + '_> {
        self.0
            .iter()
            .rev()
            .scan(vec![-1; self.0[0].len()], |max_heights, tree_heights| {
                let res = max_heights
                    .clone()
                    .into_iter()
                    .zip(tree_heights.iter())
                    .map(|(max_h, h)| *h > max_h);

                max_heights
                    .iter_mut()
                    .zip(tree_heights)
                    .for_each(|(h, h2)| {
                        *h = (*h).max(*h2);
                    });
                Some(res)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
    }

    fn compute_scenic_score(&self) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> {
        self.compute_lr_view_range()
            .zip(self.compute_rl_view_range())
            .zip(self.compute_bt_view_range())
            .zip(self.compute_tb_view_range())
            .map(|(((it1, it2), it3), it4)| {
                it1.zip(it2)
                    .zip(it3)
                    .zip(it4)
                    .map(|(((v1, v2), v3), v4)| v1 * v2 * v3 * v4)
            })
    }

    fn compute_lr_view_range(&self) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> {
        self.0.iter().map(|line| {
            line.iter().enumerate().map(move |(col_idx, &height)| {
                line.iter()
                    .skip(col_idx + 1)
                    .take_until(|&&h| h >= height)
                    .count()
            })
        })
    }

    fn compute_rl_view_range(&self) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> {
        self.0.iter().map(|line| {
            line.iter().enumerate().map(move |(col_idx, &height)| {
                line.iter()
                    .rev()
                    .skip(line.len() - col_idx)
                    .take_until(|&&h| h >= height)
                    .count()
            })
        })
    }

    fn compute_tb_view_range(&self) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> {
        self.0.iter().enumerate().map(move |(line_idx, line)| {
            line.iter().enumerate().map(move |(col_idx, &height)| {
                self.0
                    .iter()
                    .map(|line| line[col_idx])
                    .skip(line_idx + 1)
                    .take_until(|&h| h >= height)
                    .count()
            })
        })
    }

    fn compute_bt_view_range(&self) -> impl Iterator<Item = impl Iterator<Item = usize> + '_> {
        self.0.iter().enumerate().map(move |(line_idx, line)| {
            line.iter().enumerate().map(move |(col_idx, &height)| {
                self.0
                    .iter()
                    .map(|line| line[col_idx])
                    .rev()
                    .skip(self.0.len() - line_idx)
                    .take_until(|&h| h >= height)
                    .count()
            })
        })
    }
}
