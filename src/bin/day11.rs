use nom::{branch, bytes::complete::tag, combinator, multi, sequence, IResult};
use std::{cell::RefCell, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut monkeys = parse_monkeys(&input)
        .unwrap()
        .1
        .into_iter()
        .map(RefCell::new)
        .collect::<Vec<_>>();

    for _round in 0..20 {
        for monkey in &monkeys {
            for MonkeyThrow {
                dest_monkey,
                item_worry,
            } in monkey.borrow_mut().step()
            {
                monkeys[dest_monkey]
                    .borrow_mut()
                    .item_worry_levels
                    .push(item_worry);
            }
        }
    }

    monkeys.sort_by_key(|m| m.borrow().inspection_count);

    println!(
        "{}",
        monkeys
            .iter()
            .rev()
            .take(2)
            .map(|m| m.borrow().inspection_count)
            .product::<usize>()
    );
}

#[derive(Debug)]
struct MonkeyThrow {
    dest_monkey: usize,
    item_worry: i64,
}

#[derive(Debug)]
struct Monkey {
    item_worry_levels: Vec<i64>,
    worry_update: WorryUpdateExpr,
    decision_rule: MonkeyDecisionRule,

    inspection_count: usize,
}

impl Monkey {
    fn step(&mut self) -> impl Iterator<Item = MonkeyThrow> {
        for worry_level in &mut self.item_worry_levels {
            self.inspection_count += 1;
            *worry_level = self.worry_update.apply(*worry_level) / 3;
        }

        let item_worry_levels = std::mem::take(&mut self.item_worry_levels);

        let decision_rule = self.decision_rule.clone();
        item_worry_levels
            .into_iter()
            .map(move |worry_level| MonkeyThrow {
                dest_monkey: decision_rule.apply(worry_level),
                item_worry: worry_level,
            })
    }
}

#[derive(Clone, Copy, Debug)]
enum WorryUpdateExpr {
    AddConst(i64),
    MulConst(i64),
    Pow2,
}

impl WorryUpdateExpr {
    fn apply(self, worry_level: i64) -> i64 {
        match self {
            Self::Pow2 => worry_level.pow(2),
            Self::MulConst(n) => worry_level * n,
            Self::AddConst(n) => worry_level + n,
        }
    }
}

#[derive(Clone, Debug)]
struct MonkeyDecisionRule {
    test_modulus: i64,
    monkey_if_true: usize,
    monkey_if_false: usize,
}

impl MonkeyDecisionRule {
    fn apply(&self, worry_level: i64) -> usize {
        if worry_level % self.test_modulus == 0 {
            self.monkey_if_true
        } else {
            self.monkey_if_false
        }
    }
}

fn parse_worry_update_expr(input: &str) -> IResult<&str, WorryUpdateExpr> {
    sequence::preceded(
        tag("new = old "),
        branch::alt((
            combinator::map(tag("* old"), |_| WorryUpdateExpr::Pow2),
            combinator::map(
                sequence::preceded(tag("* "), nom::character::complete::i64),
                WorryUpdateExpr::MulConst,
            ),
            combinator::map(
                sequence::preceded(tag("+ "), nom::character::complete::i64),
                WorryUpdateExpr::AddConst,
            ),
        )),
    )(input)
}

fn parse_monkey_decision_rule(input: &str) -> IResult<&str, MonkeyDecisionRule> {
    combinator::map(
        sequence::tuple((
            sequence::delimited(
                sequence::pair(
                    nom::character::complete::multispace1,
                    tag("Test: divisible by "),
                ),
                nom::character::complete::i64,
                tag("\n"),
            ),
            sequence::delimited(
                sequence::pair(
                    nom::character::complete::multispace1,
                    tag("If true: throw to monkey "),
                ),
                nom::character::complete::u64,
                tag("\n"),
            ),
            sequence::delimited(
                sequence::pair(
                    nom::character::complete::multispace1,
                    tag("If false: throw to monkey "),
                ),
                nom::character::complete::u64,
                tag("\n"),
            ),
        )),
        |(test_modulus, monkey_if_true, monkey_if_false)| MonkeyDecisionRule {
            test_modulus,
            monkey_if_true: monkey_if_true as usize,
            monkey_if_false: monkey_if_false as usize,
        },
    )(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let parse_starting_items = multi::separated_list1(tag(", "), nom::character::complete::i64);

    sequence::delimited(
        sequence::tuple((tag("Monkey "), nom::character::complete::digit1, tag(":\n"))),
        combinator::map(
            sequence::tuple((
                sequence::delimited(tag("  Starting items: "), parse_starting_items, tag("\n")),
                sequence::delimited(tag("  Operation: "), parse_worry_update_expr, tag("\n")),
                parse_monkey_decision_rule,
            )),
            |(item_worry_levels, worry_update, decision_rule)| Monkey {
                item_worry_levels,
                worry_update,
                decision_rule,
                inspection_count: 0,
            },
        ),
        combinator::opt(tag("\n")),
    )(input)
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    multi::many1(parse_monkey)(input)
}
