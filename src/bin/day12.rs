use std::{collections::VecDeque, io::stdin};

fn main() {
    let puzzle = Puzzle::parse(
        stdin()
            .lines()
            .map(Result::unwrap)
            .take_while(|s| !s.is_empty()),
    );

    println!("{}", puzzle.solve_part1());
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
        let mut bfs = PuzzleBfs::new(self);
        bfs.run_to_completion();

        let (endx, endy) = self.end;
        bfs.dist_from_start[endx][endy]
    }
}

struct PuzzleBfs<'puz> {
    puzzle: &'puz Puzzle,
    prev: Vec<Vec<(usize, usize)>>,
    dist_from_start: Vec<Vec<usize>>,
}

impl<'puz> PuzzleBfs<'puz> {
    fn new(puzzle: &'puz Puzzle) -> Self {
        let n_rows = puzzle.map.len();
        let n_cols = puzzle.map[0].len();

        let prev = (0..n_rows).map(|_| vec![(0, 0); n_cols]).collect();
        let mut dist_from_start: Vec<Vec<usize>> =
            (0..n_rows).map(|_| vec![usize::MAX; n_cols]).collect();

        dist_from_start[puzzle.start.0][puzzle.start.1] = 0;

        PuzzleBfs {
            puzzle,
            prev,
            dist_from_start,
        }
    }

    fn expand<'this>(
        &'this self,
        (pos_x, pos_y): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + 'puz {
        let puz = self.puzzle;

        [(-1_isize, 0_isize), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(move |(delta_x, delta_y)| (pos_x as isize + delta_x, pos_y as isize + delta_y))
            .filter(|&(x, y)| x >= 0 && y >= 0)
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|&(x, y)| x < puz.map.len() && y < puz.map[0].len())
            .filter(move |&(x, y)| puz.map[x][y] - puz.map[pos_x][pos_y] <= 1)
    }

    fn run_to_completion(&mut self) {
        let mut queue = VecDeque::with_capacity(1);
        queue.push_back(self.puzzle.start);

        while let Some((pos_x, pos_y)) = queue.pop_front() {
            if (pos_x, pos_y) == self.puzzle.end {
                break; // reached the end
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
