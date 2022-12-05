pub fn process_part1(input: &str) -> String {
    let (stacks_input, operations) = input.split_once("\n\n").unwrap();

    let mut stack_lines = stacks_input.lines().rev();

    let stack_count = stack_lines.next().unwrap().split("   ").count();

    let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); stack_count];

    while let Some(stack_line) = stack_lines.next() {
        let stack_bytes = stack_line.bytes().collect::<Vec<u8>>();

        let mut s = 0;
        loop {
            let ci = s * 4 + 1;

            let c = stack_bytes[ci];
            if c != 32 {
                stacks[s].push(c);
            }

            s = s + 1;
            if s >= stack_count {
                break;
            }
        }
    }

    for op in operations.lines() {
        let op_parts = op.split(" ").collect::<Vec<&str>>();
        let (qty, a, b) = (
            op_parts[1].parse::<u8>().unwrap(),
            op_parts[3].parse::<usize>().unwrap(),
            op_parts[5].parse::<usize>().unwrap(),
        );

        for _ in 0..qty {
            let to_move = &stacks[a - 1].pop().unwrap();
            stacks[b - 1].push(*to_move);
        }
    }

    return String::from_utf8(stacks
        .iter()
        .map(|stack| {
            return *stack.last().unwrap();
        }).collect::<Vec<u8>>()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process_part1(TESTINPUT);
        assert_eq!(result, "CMZ");
    }

    const TESTINPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
}
