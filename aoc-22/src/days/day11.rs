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
        todo!()
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
    todo!()
}

type MonkeyId = usize;
struct Item(usize);
struct Monkey {
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    test_success_target: MonkeyId,
    test_failure_target: MonkeyId,
}

impl Monkey {
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

struct MonkeyGroup {
    monkey_items: HashMap<MonkeyId, (Monkey, VecDeque<Item>)>,
    inspect_counter: HashMap<MonkeyId, usize>,
}

impl MonkeyGroup {
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
