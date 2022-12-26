use std::{cell::RefCell, collections::VecDeque, io};

#[derive(Debug)]
enum Operation {
    Add(Option<u64>, Option<u64>),
    Mul(Option<u64>, Option<u64>),
}

impl From<&str> for Operation {
    fn from(note_line: &str) -> Self {
        let parts = note_line.split(" ").collect::<Vec<_>>();
        let left = match parts[0] {
            "old" => None,
            num => Some(num.parse::<u64>().unwrap()),
        };
        let right = match parts[2] {
            "old" => None,
            num => Some(num.parse::<u64>().unwrap()),
        };
        match parts[1] {
            "+" => Operation::Add(left, right),
            "*" => Operation::Mul(left, right),
            _ => panic!("Unknown operation"),
        }
    }
}

#[derive(Debug, Clone)]
struct TestValue {
    value: u64,
    is_divisor: bool,
    remainder: u64,
}

#[derive(Debug, Clone)]
enum Worry {
    Base(u64),
    Full(Vec<TestValue>),
}

impl From<u64> for Worry {
    fn from(value: u64) -> Self {
        Self::Base(value)
    }
}

impl Worry {
    fn convert(self: &Self, test_values: &Vec<u64>) -> Self {
        match self {
            Self::Base(val) => Self::Full(
                test_values
                    .iter()
                    .map(|tv| TestValue {
                        value: *tv,
                        is_divisor: val % tv == 0,
                        remainder: val % tv,
                    })
                    .collect::<Vec<_>>(),
            ),
            Self::Full(_) => self.clone(),
        }
    }

    fn test_for(self: &Self, test_value: u64) -> bool {
        match self {
            Self::Base(val) => val % test_value == 0,
            Self::Full(tvs) => tvs.iter().any(|tv| tv.value == test_value && tv.is_divisor),
        }
    }

    fn update(self: &Self, op: &Operation) -> Self {
        match self {
            Self::Base(val) => match op {
                Operation::Add(_, r) => Worry::from(*val + r.unwrap_or(*val)),
                Operation::Mul(_, r) => Worry::from(*val * r.unwrap_or(*val)),
            },
            Self::Full(tvs) => Self::Full(
                tvs.iter()
                    .map(|tv| match op {
                        Operation::Add(_, r) => {
                            let new_remainder =
                                (tv.remainder + r.unwrap_or(tv.remainder) % tv.value) % tv.value;
                            TestValue {
                                value: tv.value,
                                is_divisor: new_remainder == 0,
                                remainder: new_remainder,
                            }
                        }
                        Operation::Mul(_, r) => {
                            let new_remainder =
                                (tv.remainder * r.unwrap_or(tv.remainder) % tv.value) % tv.value;
                            TestValue {
                                value: tv.value,
                                is_divisor: new_remainder == 0,
                                remainder: new_remainder,
                            }
                        }
                    })
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: RefCell<VecDeque<Worry>>,
    operation: Operation,
    test_value: u64,
    positive_test_target: usize,
    negative_test_target: usize,
    inspected_item_count: RefCell<u128>,
}

impl From<&[String]> for Monkey {
    fn from(notes_descriptor: &[String]) -> Self {
        Monkey {
            items: notes_descriptor[1].split(": ").collect::<Vec<_>>()[1]
                .split(", ")
                .map(str::parse::<u64>)
                .map(Result::unwrap)
                .map(Worry::from)
                .collect::<VecDeque<_>>()
                .into(),
            operation: Operation::from(notes_descriptor[2].split("= ").collect::<Vec<_>>()[1]),
            test_value: notes_descriptor[3]
                .split(" ")
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            positive_test_target: notes_descriptor[4]
                .split(" ")
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            negative_test_target: notes_descriptor[5]
                .split(" ")
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            inspected_item_count: 0.into(),
        }
    }
}

impl Monkey {
    fn inspect_items(self: &Self, monkeys: &Vec<Monkey>) -> () {
        while !self.items.borrow().is_empty() {
            *self.inspected_item_count.borrow_mut() += 1;
            let new_worry = self
                .items
                .borrow_mut()
                .pop_front()
                .unwrap()
                .update(&self.operation);
            if new_worry.test_for(self.test_value) {
                monkeys[self.positive_test_target]
                    .items
                    .borrow_mut()
                    .push_back(new_worry);
            } else {
                monkeys[self.negative_test_target]
                    .items
                    .borrow_mut()
                    .push_back(new_worry);
            }
        }
    }
}

use crate::input_reader;
pub fn solve() -> io::Result<()> {
    let lines = input_reader::read_input("input/day11.in")?;

    let mut monkeys = lines.chunks(7).map(Monkey::from).collect::<Vec<_>>();
    let test_values = monkeys.iter().map(|m| m.test_value).collect::<Vec<_>>();
    for monkey in &monkeys {
        let new_items = monkey
            .items
            .borrow()
            .iter()
            .map(|w| w.convert(&test_values))
            .collect::<VecDeque<_>>();
        *monkey.items.borrow_mut() = new_items;
    }

    for _ in 0..10000 {
        for monkey in &monkeys {
            monkey.inspect_items(&monkeys);
        }
    }

    monkeys.sort_by_key(|x| *x.inspected_item_count.borrow());
    monkeys.reverse();

    println!(
        "Level of monkey business: {}",
        *monkeys[0].inspected_item_count.borrow() * *monkeys[1].inspected_item_count.borrow()
    );

    Ok(())
}
