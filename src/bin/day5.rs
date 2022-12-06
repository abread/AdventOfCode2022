use std::str::FromStr;

fn main() {
    let puzzle = Puzzle::parse_from(std::io::stdin().lines().map(Result::unwrap)).unwrap();

    dbg!(
        "part1",
        puzzle
            .clone()
            .execute()
            .expect("wrong instructions")
            .into_iter()
            .filter_map(|mut stack| stack.pop())
            .map(|c| c.0)
            .collect::<String>()
    );

    dbg!(
        "part2",
        puzzle
            .execute_v2()
            .expect("wrong instructions")
            .into_iter()
            .filter_map(|mut stack| stack.pop())
            .map(|c| c.0)
            .collect::<String>()
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Puzzle {
    stacks: Vec<Vec<Crate>>,
    program: Vec<MoveInstruction>,
}

impl Puzzle {
    fn parse_from(mut input: impl Iterator<Item = String>) -> Result<Self, ParseError> {
        let stacks = parse_stacks(&mut input)?;

        let empty_line = input.next();
        if empty_line.is_none() || !empty_line.unwrap().is_empty() {
            return Err(ParseError);
        }

        let program = input
            .take_while(|l| !l.is_empty())
            .map(|l| l.parse::<MoveInstruction>())
            .collect::<Result<Vec<_>, _>>()?;

        for instr in &program {
            if instr.from >= stacks.len() || instr.to >= stacks.len() {
                return Err(ParseError);
            }
        }

        Ok(Puzzle { stacks, program })
    }

    fn execute(mut self) -> Option<Vec<Vec<Crate>>> {
        for instr in self.program {
            for _ in 0..instr.amount {
                let el = self.stacks[instr.from].pop()?;
                self.stacks[instr.to].push(el);
            }
        }

        Some(self.stacks)
    }

    fn execute_v2(mut self) -> Option<Vec<Vec<Crate>>> {
        for instr in self.program {
            let split_idx = self.stacks[instr.from].len() - instr.amount;
            let els = self.stacks[instr.from].split_off(split_idx);
            self.stacks[instr.to].extend(els);
        }

        Some(self.stacks)
    }
}

fn parse_stacks(mut input: impl Iterator<Item = String>) -> Result<Vec<Vec<Crate>>, ParseError> {
    let mut line = input.next().ok_or(ParseError)?;
    if line.len() < 2 {
        return Err(ParseError);
    }

    let mut stacks = Vec::with_capacity((line.len() + 1) / 4);
    stacks.resize_with((line.len() + 1) / 4, Vec::new);
    let desired_len = line.len();

    while line.as_bytes()[0] == b'[' || line.as_bytes()[1] == b' ' {
        #[derive(PartialEq)]
        enum State {
            Init,

            EmptyChar,
            EmptyRBracket,

            OccupiedChar,
            OccupiedRBracket,

            Space,
        }
        use State::*;

        let mut stack_idx = 0_usize;
        let mut state = Init;
        for c in line.chars() {
            state = match (state, c) {
                (Init, ' ') => EmptyChar,
                (Init, '[') => OccupiedChar,

                (EmptyChar, ' ') => {
                    // nothing to push
                    stack_idx += 1;
                    EmptyRBracket
                }
                (EmptyRBracket, ' ') => Space,

                (OccupiedChar, c) => {
                    stacks[stack_idx].push(Crate(c));
                    stack_idx += 1;
                    OccupiedRBracket
                }
                (OccupiedRBracket, ']') => Space,

                (Space, ' ') => Init,

                _ => return Err(ParseError),
            };
        }

        if state != Space {
            return Err(ParseError);
        }

        line = input.next().ok_or(ParseError)?;
        if line.len() != desired_len {
            return Err(ParseError);
        }
    }

    #[derive(PartialEq)]
    enum LastLineState {
        SpaceL,
        N,
        SpaceR,
        Space,
    }
    use LastLineState::*;

    let mut n = b'0'; // ugly hack
    let mut state = SpaceL;
    for c in line.bytes() {
        (n, state) = match (n, state, c) {
            (n, SpaceL, b' ') => (n, N),
            (n, N, next_n) if next_n == n + 1 => (next_n, SpaceR),
            (n, SpaceR, b' ') => (n, Space),
            (n, Space, b' ') => (n, SpaceL),
            _ => return Err(ParseError),
        }
    }

    for st in &mut stacks {
        st.reverse();
    }
    Ok(stacks)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MoveInstruction {
    from: usize,
    to: usize,
    amount: usize,
}

impl FromStr for MoveInstruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[derive(PartialEq)]
        enum State {
            TokMove,
            Amount,
            TokFrom(usize),
            From(usize),
            TokTo(usize, usize),
            To(usize, usize),
            Done(usize, usize, usize),
        }
        use State::*;

        let mut state = TokMove;
        for tok in s.split(' ') {
            state = match (state, tok) {
                (TokMove, "move") => Amount,
                (Amount, amount) => {
                    let amount = amount.parse().map_err(|_| ParseError)?;
                    TokFrom(amount)
                }
                (TokFrom(a), "from") => From(a),
                (From(a), from) => {
                    let from: usize = from.parse().map_err(|_| ParseError)?;
                    TokTo(a, from - 1)
                }
                (TokTo(a, f), "to") => To(a, f),
                (To(a, f), to) => {
                    let to: usize = to.parse().map_err(|_| ParseError)?;
                    Done(a, f, to - 1)
                }

                _ => return Err(ParseError),
            }
        }

        if let Done(amount, from, to) = state {
            Ok(MoveInstruction { from, to, amount })
        } else {
            Err(ParseError)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Crate(char);

#[derive(Debug)]
struct ParseError;
