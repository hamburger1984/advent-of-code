extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::time::Instant;

use std::collections::VecDeque;

pub fn process_part1(input: &str) -> String {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey_str in input.split("\n\n") {
        let mut items = VecDeque::new();
        let mut opt_operation: Option<(String, String)> = None;
        let mut opt_divisor: Option<u64> = None;
        let mut opt_target_true: Option<usize> = None;
        let mut opt_target_false: Option<usize> = None;

        for line in monkey_str.lines().skip(1) {
            match line.trim().split(" ").collect::<Vec<&str>>().as_slice() {
                ["Operation:", "new", "=", "old", op1, op2] => {
                    //print!("OP      {}{}\n", op1, op2);
                    opt_operation = Some((op1.to_string(), op2.to_string()));
                }
                ["Test:", "divisible", "by", test_value] => {
                    //print!("TS      >> {}\n", test_value);
                    opt_divisor = Some(test_value.parse().unwrap());
                }
                ["If", "true:", "throw", "to", "monkey", target] => {
                    //print!("YES     >> {}\n", target);
                    opt_target_true = Some(target.parse().unwrap());
                }
                ["If", "false:", "throw", "to", "monkey", target] => {
                    //print!("NO     >> {}\n", target);
                    opt_target_false = Some(target.parse().unwrap());
                }
                other => {
                    //print!("Items {:?}\n", other);
                    for item in other.into_iter().skip(2) {
                        items.push_back(item.trim_end_matches(',').parse().unwrap());
                    }
                }
            }
        }

        if let Some(operation) = opt_operation {
            if let Some(divisor) = opt_divisor {
                if let Some(target_true) = opt_target_true {
                    if let Some(target_false) = opt_target_false {
                        monkeys.push(Monkey {
                            items,
                            operation,
                            divisor,
                            target_true,
                            target_false,
                            activities: 0,
                        });
                        continue;
                    }
                }
            }
        }
        panic!("Incomplete Monkey!");
    }

    let rounds = 20;
    let monkey_count = monkeys.len();

    for round in 1..=rounds {
        print!("Round {}\n", round);

        for i_monkey in 0..monkey_count {
            //print!("Monkey {}\n", i_monkey);

            let (left, right) = monkeys.split_at_mut(i_monkey);
            while !right[0].items.is_empty() {
                if let Some(item) = right[0].items.pop_front() {
                    //print!(" Inspect {}\n", item);
                    let new_level = apply_operation(item, &right[0].operation);
                    //print!("  New level {}\n", new_level);

                    let target_monkey = match new_level % right[0].divisor == 0 {
                        true => right[0].target_true,
                        false => right[0].target_false,
                    };

                    //print!("  Throw to {}\n", target_monkey);
                    if target_monkey > i_monkey {
                        right[target_monkey - i_monkey].items.push_back(new_level);
                    } else {
                        left[target_monkey].items.push_back(new_level);
                    }

                    right[0].activities += 1;
                }
            }
        }
    }

    let mut activities: Vec<u64> = monkeys.iter().map(|m| m.activities).collect();
    activities.sort_by(|a, b| b.cmp(a));

    return (activities[0] * activities[1]).to_string();
}

pub fn process_part2(input: &str) -> String {
    let mut monkeys: Vec<Monkey2> = Vec::new();
    let mut monkey_items: Vec<VecDeque<BigUint>> = Vec::new();

    for monkey_str in input.split("\n\n") {
        let mut items = VecDeque::new();
        let mut opt_operation: Option<(bool, u64)> = None;
        let mut opt_divisor: Option<u64> = None;
        let mut opt_target_true: Option<usize> = None;
        let mut opt_target_false: Option<usize> = None;

        for line in monkey_str.lines().skip(1) {
            match line.trim().split(" ").collect::<Vec<&str>>().as_slice() {
                ["Operation:", "new", "=", "old", op1, op2] => {
                    let is_multiplication = (*op1).eq("*");
                    let value = match (*op2).eq("old") {
                        true => u64::MAX,
                        false => op2.parse().unwrap(),
                    };
                    opt_operation = Some((is_multiplication, value));
                }
                ["Test:", "divisible", "by", test_value] => {
                    opt_divisor = Some(test_value.parse().unwrap());
                }
                ["If", "true:", "throw", "to", "monkey", target] => {
                    opt_target_true = Some(target.parse().unwrap());
                }
                ["If", "false:", "throw", "to", "monkey", target] => {
                    opt_target_false = Some(target.parse().unwrap());
                }
                other => {
                    for item in other.into_iter().skip(2) {
                        items.push_back(item.trim_end_matches(',').parse().unwrap());
                    }
                }
            }
        }

        if let Some(operation) = opt_operation {
            if let Some(divisor) = opt_divisor {
                if let Some(target_true) = opt_target_true {
                    if let Some(target_false) = opt_target_false {
                        monkeys.push(Monkey2 {
                            operation,
                            divisor,
                            target_true,
                            target_false,
                            activities: 0,
                        });
                        monkey_items.push(items);
                        continue;
                    }
                }
            }
        }
        panic!("Incomplete Monkey!");
    }

    let divisors = monkeys.iter().map(|m| m.divisor);

    let mut trimmer: BigUint = One::one();
    for d in divisors {
        trimmer *= d;
    }

    let rounds = 10_000 as u16;
    let monkey_count = monkeys.len();

    let start = Instant::now();

    let mutmonkey = monkeys.as_mut_slice();

    for round in 1..=rounds {
        for i_monkey in 0..monkey_count {
            while !monkey_items[i_monkey].is_empty() {
                let mut inspected_item = monkey_items[i_monkey].pop_front().unwrap();

                match mutmonkey[i_monkey].operation.1 {
                    u64::MAX => match mutmonkey[i_monkey].operation.0 {
                        true => inspected_item = &inspected_item * &inspected_item,
                        false => inspected_item = &inspected_item + &inspected_item,
                    },
                    number => match mutmonkey[i_monkey].operation.0 {
                        true => inspected_item *= number,
                        false => inspected_item += number,
                    },
                };

                let target_monkey =
                    match &inspected_item % mutmonkey[i_monkey].divisor == Zero::zero() {
                        true => mutmonkey[i_monkey].target_true,
                        false => mutmonkey[i_monkey].target_false,
                    };

                monkey_items[target_monkey].push_back(inspected_item % &trimmer);

                mutmonkey[i_monkey].activities += 1;
            }
        }

        if round % 50 == 0 {
            print!("Round {} .. {}\n", round, start.elapsed().as_secs_f32());
        }
    }

    let mut activities: Vec<u64> = monkeys.iter().map(|m| m.activities).collect();
    activities.sort_by(|a, b| b.cmp(a));

    return (activities[0] * activities[1]).to_string();
}

fn apply_operation(level: u64, operation: &(String, String)) -> u64 {
    let b = match operation.1.as_str() {
        "old" => level,
        number => number.parse().unwrap(),
    };

    return match operation.0.as_str() {
        "*" => (level * b) / 3,
        "+" => (level + b) / 3,
        _ => panic!("Unknown operation"),
    };
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: (String, String),
    divisor: u64,
    target_true: usize,
    target_false: usize,
    activities: u64,
}

#[derive(Debug)]
struct Monkey2 {
    operation: (bool, u64),
    divisor: u64,
    target_true: usize,
    target_false: usize,
    activities: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, "10605");
    }

    #[test]
    fn test_part2() {
        let result = process_part2(TEST_INPUT);
        assert_eq!(result, "2713310158");
    }

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}
