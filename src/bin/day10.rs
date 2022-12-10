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

    println!("{}", part1_solution);
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
            match self.ops_iter.next() {
                None => return Err(()),
                Some(x) => {
                    self.state.update(x);
                }
            }
        }

        Ok(())
    }

    fn state(&self) -> &SimulationState {
        &self.state
    }
}

#[derive(Debug, PartialEq)]
struct SimulationState {
    register_x: i64,
    started_cycle_counter: usize,
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
        }
    }
}

impl Default for SimulationState {
    fn default() -> Self {
        SimulationState {
            register_x: 1,
            started_cycle_counter: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SimOp {
    AddX(i64),
    IncrStartedCycleCounter,
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
                    SimOp::IncrStartedCycleCounter,
                    SimOp::AddX(amount),
                ]
                .into_iter(),
            ),
            Instruction::Noop => Box::new([SimOp::IncrStartedCycleCounter].into_iter()),
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
