use std::{collections::VecDeque, io::stdin};

fn main() {
    let puzzle = Puzzle::parse(
        stdin()
            .lines()
            .map(Result::unwrap)
            .take_while(|s| !s.is_empty()),
    );

    println!("{}", puzzle.solve_part1());
    println!("{}", puzzle.solve_part2());
}

#[derive(Debug)]
struct Puzzle {
    map: Vec<Vec<i8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Puzzle {
    fn parse(input: impl Iterator<Item = String>) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);

        let map = input
            .enumerate()
            .map(|(x, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(y, val)| match val {
                        b'a'..=b'z' => (val - b'a') as i8,
                        b'S' => {
                            start = (x, y);
                            0
                        }
                        b'E' => {
                            end = (x, y);
                            (b'z' - b'a') as i8
                        }
                        _ => unreachable!("invalid input"),
                    })
                    .collect()
            })
            .collect();

        Puzzle { map, start, end }
    }

    fn solve_part1(&self) -> usize {
        let neigh_filter = |(neigh_x, neigh_y): (usize, usize), (px, py): (usize, usize)| {
            self.map[neigh_x][neigh_y] - self.map[px][py] <= 1
        };
        let mut bfs = PuzzleBfs::new(self, &neigh_filter);

        let stop_cond = |coords| coords == self.end;
        bfs.run_to_completion(self.start, stop_cond);

        let (endx, endy) = self.end;
        bfs.dist_from_start[endx][endy]
    }

    fn solve_part2(&self) -> usize {
        let neigh_filter = |(neigh_x, neigh_y): (usize, usize), (px, py): (usize, usize)| {
            self.map[px][py] - self.map[neigh_x][neigh_y] <= 1
        };
        let mut bfs = PuzzleBfs::new(self, &neigh_filter);

        let stop_cond = |_| false;
        bfs.run_to_completion(self.end, stop_cond);

        let possible_start_positions = self.map.iter().enumerate().flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(y, &h)| if h == 0 { Some((x, y)) } else { None })
        });

        possible_start_positions
            .map(|(x, y)| bfs.dist_from_start[x][y])
            .min()
            .unwrap()
    }
}

struct PuzzleBfs<'nf, NF> {
    prev: Vec<Vec<(usize, usize)>>,
    dist_from_start: Vec<Vec<usize>>,
    neighbor_filter: &'nf NF,
}

impl<'nf, NF> PuzzleBfs<'nf, NF>
where
    NF: Fn((usize, usize), (usize, usize)) -> bool,
{
    fn new<'puz>(puzzle: &'puz Puzzle, neighbor_filter: &'nf NF) -> Self {
        let n_rows = puzzle.map.len();
        let n_cols = puzzle.map[0].len();

        let prev = (0..n_rows).map(|_| vec![(0, 0); n_cols]).collect();
        let dist_from_start: Vec<Vec<usize>> =
            (0..n_rows).map(|_| vec![usize::MAX; n_cols]).collect();

        PuzzleBfs {
            prev,
            dist_from_start,
            neighbor_filter,
        }
    }

    fn expand(&self, (pos_x, pos_y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + 'nf {
        let n_rows = self.dist_from_start.len();
        let n_cols = self.dist_from_start[0].len();
        let nf = self.neighbor_filter;

        [(-1_isize, 0_isize), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(move |(delta_x, delta_y)| (pos_x as isize + delta_x, pos_y as isize + delta_y))
            .filter(|&(x, y)| x >= 0 && y >= 0)
            .map(|(x, y)| (x as usize, y as usize))
            .filter(move |&(x, y)| x < n_rows && y < n_cols)
            .filter(move |&(x, y)| nf((x, y), (pos_x, pos_y)))
    }

    fn run_to_completion(
        &mut self,
        start: (usize, usize),
        stop_condition: impl Fn((usize, usize)) -> bool,
    ) {
        let mut queue = VecDeque::with_capacity(1);
        queue.push_back(start);

        self.dist_from_start[start.0][start.1] = 0;
        while let Some((pos_x, pos_y)) = queue.pop_front() {
            if stop_condition((pos_x, pos_y)) {
                break;
            }

            for (neigh_x, neigh_y) in self.expand((pos_x, pos_y)) {
                if self.dist_from_start[neigh_x][neigh_y] > self.dist_from_start[pos_x][pos_y] + 1 {
                    self.dist_from_start[neigh_x][neigh_y] = self.dist_from_start[pos_x][pos_y] + 1;
                    self.prev[neigh_x][neigh_y] = (pos_x, pos_y);

                    queue.push_back((neigh_x, neigh_y));
                }
            }
        }
    }
}
