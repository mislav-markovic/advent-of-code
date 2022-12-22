use crate::day_exec::DayExecutor;

use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    str::FromStr,
};

pub struct Day11;

impl DayExecutor for Day11 {
    fn exec_part1(&self, input: String) -> Box<dyn Display> {
        Box::new(format!("Monkey business score: {}", solve_part1(&input)))
    }

    fn exec_part2(&self, input: String) -> Box<dyn Display> {
        Box::new(format!("Monkey business score: TODO"))
    }
}

fn solve_part1(input: &str) -> usize {
    let mut group = get_monkey_group_from_input(input);

    for _ in 0..20 {
        group.play_round();
    }

    let mut num_of_inspects = group.inspect_counter.values().collect::<Vec<_>>();
    num_of_inspects.sort();
    let last = num_of_inspects.pop().unwrap();
    let second_to_last = num_of_inspects.pop().unwrap();

    last * second_to_last
}

fn get_monkey_group_from_input(input: &str) -> MonkeyGroup {
    input
        .parse::<MonkeyGroup>()
        .expect("Failed to parse monkey group")
}

type MonkeyId = usize;
#[derive(Debug)]
struct Item(usize);
struct Monkey {
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    test_success_target: MonkeyId,
    test_failure_target: MonkeyId,
}

impl Monkey {
    fn new<OpF, TestF>(
        op: OpF,
        test: TestF,
        test_success_target: MonkeyId,
        test_failure_target: MonkeyId,
    ) -> Self
    where
        OpF: Fn(usize) -> usize + 'static,
        TestF: Fn(usize) -> bool + 'static,
    {
        let operation = Box::new(op);
        let test = Box::new(test);

        Self {
            operation,
            test,
            test_success_target,
            test_failure_target,
        }
    }

    fn inspect(&self, mut item: Item) -> (Item, MonkeyId) {
        item.0 = (self.operation)(item.0);
        item.0 /= 3;

        let target = if (self.test)(item.0) {
            self.test_success_target
        } else {
            self.test_failure_target
        };

        (item, target)
    }
}

struct MonkeyTriplet(MonkeyId, Monkey, VecDeque<Item>);
#[derive(Debug)]
struct MonkeyParseError;
impl FromStr for MonkeyTriplet {
    type Err = MonkeyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let monkey_id_str = lines.next().ok_or(MonkeyParseError)?;
        let starting_items_str = lines.next().ok_or(MonkeyParseError)?;
        let operation_str = lines.next().ok_or(MonkeyParseError)?;
        let test_str = lines.next().ok_or(MonkeyParseError)?;
        let success_target_str = lines.next().ok_or(MonkeyParseError)?;
        let failure_target_str = lines.next().ok_or(MonkeyParseError)?;

        let monkey_id = monkey_id_str
            .trim()
            .trim_start_matches("Monkey")
            .trim_end_matches(":")
            .trim()
            .parse::<MonkeyId>()
            .map_err(|_| MonkeyParseError {})?;

        let starting_items = starting_items_str
            .trim()
            .trim_start_matches("Starting items:")
            .split(',')
            .map(|s| s.trim().parse::<usize>().map(|v| Item(v)))
            .collect::<Result<VecDeque<_>, _>>()
            .map_err(|_| MonkeyParseError {})?;

        let (_, formula) = operation_str.split_once('=').ok_or(MonkeyParseError)?;
        let mut parts = formula.split_ascii_whitespace();
        let lhs = parts.next().ok_or(MonkeyParseError)?;
        let op = parts.next().ok_or(MonkeyParseError)?;
        let rhs = parts.next().ok_or(MonkeyParseError)?;

        let lhs = lhs.parse::<Variable>().map_err(|_| MonkeyParseError)?;
        let op = op.parse::<Operand>().map_err(|_| MonkeyParseError)?;
        let rhs = rhs.parse::<Variable>().map_err(|_| MonkeyParseError)?;

        let operation = move |item: usize| op.calc(lhs.get_val(item), rhs.get_val(item));

        let test_divisible_by =
            test_str
                .split_once("by")
                .ok_or(MonkeyParseError)
                .and_then(|(_, divider)| {
                    divider
                        .trim()
                        .parse::<usize>()
                        .map_err(|_| MonkeyParseError {})
                })?;

        let test = move |item: usize| (item % test_divisible_by) == 0;

        let success_target_id = success_target_str
            .trim()
            .split_ascii_whitespace()
            .last()
            .ok_or(MonkeyParseError)
            .and_then(|id_str| id_str.parse::<MonkeyId>().map_err(|_| MonkeyParseError {}))?;

        let failure_target_id = failure_target_str
            .trim()
            .split_ascii_whitespace()
            .last()
            .ok_or(MonkeyParseError)
            .and_then(|id_str| id_str.parse::<MonkeyId>().map_err(|_| MonkeyParseError {}))?;

        let monkey = Monkey::new(operation, test, success_target_id, failure_target_id);
        Ok(MonkeyTriplet(monkey_id, monkey, starting_items))
    }
}

#[derive(Debug)]
enum Operand {
    Add,
    Mul,
}

impl Operand {
    fn calc(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Operand::Add => lhs + rhs,
            Operand::Mul => lhs * rhs,
        }
    }
}

struct OperandParseError;
impl FromStr for Operand {
    type Err = OperandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "*" => Ok(Self::Mul),
            "+" => Ok(Self::Add),
            _ => Err(OperandParseError),
        }
    }
}

#[derive(Debug)]
enum Variable {
    Constant(usize),
    Old,
}

impl Variable {
    fn get_val(&self, current_val: usize) -> usize {
        match self {
            Variable::Constant(constant_val) => *constant_val,
            Variable::Old => current_val,
        }
    }
}

struct VariableParseError;
impl FromStr for Variable {
    type Err = VariableParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        if trimmed == "old" {
            Ok(Self::Old)
        } else {
            trimmed
                .parse::<usize>()
                .map(|v| Self::Constant(v))
                .map_err(|_| VariableParseError {})
        }
    }
}

struct MonkeyGroup {
    monkey_items: HashMap<MonkeyId, (Monkey, VecDeque<Item>)>,
    inspect_counter: HashMap<MonkeyId, usize>,
}

impl MonkeyGroup {
    fn new() -> Self {
        Self {
            monkey_items: HashMap::new(),
            inspect_counter: HashMap::new(),
        }
    }

    fn add_monkey(&mut self, to_add: MonkeyTriplet) {
        self.monkey_items.insert(to_add.0, (to_add.1, to_add.2));
        self.inspect_counter.insert(to_add.0, 0);
    }

    fn play_round(&mut self) {
        let mut turn_order = self.monkey_items.keys().cloned().collect::<Vec<_>>();
        turn_order.sort();

        for monkey_id in turn_order {
            self.play_turn(&monkey_id);
        }
    }

    fn play_turn(&mut self, for_monkey: &MonkeyId) {
        let (monkey, items) = self.monkey_items.get_mut(for_monkey).unwrap();
        let mut thrown_items: Vec<(Item, MonkeyId)> = Vec::with_capacity(items.len());

        while let Some(item) = items.pop_front() {
            thrown_items.push(monkey.inspect(item));

            self.inspect_counter
                .entry(*for_monkey)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        for (item, target) in thrown_items {
            self.monkey_items
                .entry(target)
                .and_modify(|(_, items)| items.push_back(item));
        }
    }
}

#[derive(Debug)]
struct MonkeyGroupParseError;
impl FromStr for MonkeyGroup {
    type Err = MonkeyGroupParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut group = Self::new();

        s.split("\n\n")
            .map(|m| {
                m.parse::<MonkeyTriplet>()
                    .map_err(|_| MonkeyGroupParseError)
            })
            .try_for_each(|m| -> Result<(), MonkeyGroupParseError> {
                let inner = m?;
                group.add_monkey(inner);
                Ok(())
            })?;

        Ok(group)
    }
}
