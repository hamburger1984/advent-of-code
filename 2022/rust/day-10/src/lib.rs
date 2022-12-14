use std::process::exit;

pub fn process_part1(input:&str) -> String {
    let mut register_x = 1;
    let mut cycle = 1;

    let mut signal_sum = 0;

    for line in input.lines() {
        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["noop"] => {
                if (cycle + 20) % 40 == 0 {
                    signal_sum += cycle * register_x;
                    print!("{}: sum={}\n", cycle, signal_sum);
                }
                cycle += 1;
            }
            ["addx", next_add] => {
                let next_add_value:i64 = next_add.parse().unwrap();
                if (cycle + 20) % 40 == 0 {
                    signal_sum += cycle * register_x;
                    print!("{}: sum={}\n", cycle, signal_sum);
                }
                cycle += 1;
                if (cycle + 20) % 40 == 0 {
                    signal_sum += cycle * register_x;
                    print!("{}: sum={}\n", cycle, signal_sum);
                }
                register_x += next_add_value;
                cycle += 1;
            }
            _ => {exit(1)}
        }
    }

    return signal_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, "13140");
    }

    const TEST_INPUT:&str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
