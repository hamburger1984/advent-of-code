use std::collections::VecDeque;

pub fn process_part1(input: &str) -> String {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey_str in input.split("\n\n") {
        for line in monkey_str.lines().skip(1) {
            match line.trim().split(" ").collect::<Vec<&str>>().as_slice() {
                ["Operation:", "new", "=", op1, operand, op2] => {
                    print!("OP      {}{}{}\n", op1, operand, op2);
                }
                ["Test:", "divisible", "by", test_value] => {
                    print!("TS      >> {}\n", test_value);
                }
                ["If", "true:", "throw", "to", "monkey", target] => {
                    print!("YES     >> {}\n", target);
                }
                ["If", "false:", "throw", "to", "monkey", target] => {
                    print!("NO     >> {}\n", target);
                }
                ["Starting", "items:", items] => {
                    print!("ITEMS   {}\n", items);
                }
                other => {
                    print!(" -- {:?}\n", other);
                }
            }
        }
    }

    return 1.to_string();
}

struct Monkey {
    items: VecDeque<u64>,
    operation: (String, String, String),
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
