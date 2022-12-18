use std::str::FromStr;

fn main() {
    let simulation_ops = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Instruction>().unwrap())
        .flat_map(Instruction::to_simulation_ops);

    let mut sim = Simulation {
        ops_iter: simulation_ops,
        state: SimulationState::default(),
    };

    let mut part1_solution = 0;
    for inspection_point in [20, 60, 100, 140, 180, 220] {
        sim.step_until(|st| st.started_cycle_counter == inspection_point)
            .expect("not enough instructions");

        let part1_signal_strength = (inspection_point as i64) * sim.state().register_x;
        part1_solution += part1_signal_strength;
    }
    println!("{part1_solution}");

    sim.step_remaining();
    for line in sim.state().crt.chunks_exact(CRT_COLUMNS) {
        for c in line.iter().map(|is_lit| if *is_lit { '#' } else { '.' }) {
            print!("{c}");
        }

        println!();
    }
}

struct Simulation<OpIt> {
    ops_iter: OpIt,
    state: SimulationState,
}

impl<OpIt> Simulation<OpIt>
where
    OpIt: Iterator<Item = SimOp>,
{
    fn step_until(&mut self, mut cond: impl FnMut(&SimulationState) -> bool) -> Result<(), ()> {
        while !cond(&self.state) {
            self.step_one()?;
        }

        Ok(())
    }

    fn step_remaining(&mut self) {
        while self.step_one().is_ok() {}
    }

    fn step_one(&mut self) -> Result<(), ()> {
        match self.ops_iter.next() {
            None => return Err(()),
            Some(x) => {
                self.state.update(x);
            }
        }

        Ok(())
    }

    fn state(&self) -> &SimulationState {
        &self.state
    }
}

const CRT_COLUMNS: usize = 40;
const CRT_ROWS: usize = 6;

#[derive(Debug, PartialEq)]
struct SimulationState {
    register_x: i64,
    started_cycle_counter: usize,

    crt: [bool; CRT_ROWS * CRT_COLUMNS],
    crt_electron_gun_pos: usize,
}

impl SimulationState {
    fn update(&mut self, op: SimOp) {
        match op {
            SimOp::IncrStartedCycleCounter => {
                self.started_cycle_counter += 1;
            }
            SimOp::AddX(amount) => {
                self.register_x += amount;
            }
            SimOp::UpdateCrt => {
                let gun_column = (self.crt_electron_gun_pos % CRT_COLUMNS) as i64;
                let sprite_pos = (self.register_x - 1)..=(self.register_x + 1);
                let is_lit = sprite_pos.contains(&gun_column);
                self.crt[self.crt_electron_gun_pos] = is_lit;

                self.crt_electron_gun_pos =
                    (self.crt_electron_gun_pos + 1) % (CRT_ROWS * CRT_COLUMNS);
            }
        }
    }
}

impl Default for SimulationState {
    fn default() -> Self {
        SimulationState {
            register_x: 1,
            started_cycle_counter: 0,

            crt: [false; CRT_ROWS * CRT_COLUMNS],
            crt_electron_gun_pos: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SimOp {
    AddX(i64),
    IncrStartedCycleCounter,
    UpdateCrt,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Instruction {
    AddX(i64),
    Noop,
}

impl Instruction {
    fn to_simulation_ops(self) -> Box<dyn Iterator<Item = SimOp>> {
        match self {
            Instruction::AddX(amount) => Box::new(
                [
                    SimOp::IncrStartedCycleCounter,
                    SimOp::UpdateCrt,
                    SimOp::IncrStartedCycleCounter,
                    SimOp::UpdateCrt,
                    SimOp::AddX(amount),
                ]
                .into_iter(),
            ),
            Instruction::Noop => {
                Box::new([SimOp::IncrStartedCycleCounter, SimOp::UpdateCrt].into_iter())
            }
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

#[derive(Debug)]
struct ParseErr;
