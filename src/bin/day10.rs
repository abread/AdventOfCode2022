use std::str::FromStr;

fn main() {
    let mut program_uops = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Instruction>().unwrap())
        .flat_map(Instruction::to_micro_ops);

    let mut cpu_state = CpuState::default();

    let mut part1_solution = 0;
    for inspection_point in [20, 60, 100, 140, 180, 220] {
        cpu_state = program_uops
            .fold_n(
                inspection_point - cpu_state.cycle_counter - 1,
                cpu_state,
                CpuState::update,
            )
            .expect("not enough instructions");

        debug_assert_eq!(cpu_state.cycle_counter + 1, inspection_point);

        let part1_signal_strength = (inspection_point as i64) * cpu_state.register_x;
        part1_solution += part1_signal_strength;
    }

    println!("{}", part1_solution);
}

#[derive(Debug, PartialEq)]
struct CpuState {
    register_x: i64,
    cycle_counter: usize,
}

impl CpuState {
    fn update(mut self, op: MicroOp) -> CpuState {
        self.cycle_counter += 1;

        match op {
            MicroOp::Noop => (),
            MicroOp::AddX(amount) => {
                self.register_x += amount;
            }
        }

        self
    }
}

impl Default for CpuState {
    fn default() -> Self {
        CpuState {
            register_x: 1,
            cycle_counter: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MicroOp {
    AddX(i64),
    Noop,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Instruction {
    AddX(i64),
    Noop,
}

impl Instruction {
    fn to_micro_ops(self) -> Box<dyn Iterator<Item = MicroOp>> {
        match self {
            Instruction::AddX(amount) => {
                Box::new([MicroOp::Noop, MicroOp::AddX(amount)].into_iter())
            }
            Instruction::Noop => Box::new([MicroOp::Noop].into_iter()),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        let instr = match tokens.next().ok_or(ParseErr)? {
            "noop" => Instruction::Noop,
            "addx" => {
                let amount = tokens.next().ok_or(ParseErr)?;
                let amount = amount.parse().map_err(|_| ParseErr)?;

                Instruction::AddX(amount)
            }
            _ => return Err(ParseErr),
        };

        Ok(instr)
    }
}

trait FoldNExt: Iterator {
    fn fold_n<B: std::fmt::Debug>(
        &mut self,
        n: usize,
        init: B,
        folder: impl FnMut(B, Self::Item) -> B,
    ) -> Option<B>;
}

impl<It: Iterator> FoldNExt for It {
    fn fold_n<B: std::fmt::Debug>(
        &mut self,
        mut n: usize,
        init: B,
        mut folder: impl FnMut(B, Self::Item) -> B,
    ) -> Option<B> {
        let mut accum = init;

        while n > 0 {
            match self.next() {
                Some(x) => {
                    accum = folder(accum, x);
                }
                None => return None,
            }

            n -= 1;
        }

        Some(accum)
    }
}

#[derive(Debug)]
struct ParseErr;
