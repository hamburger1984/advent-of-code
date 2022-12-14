use std::process::exit;

pub fn process_part1(input:&str) -> String {
    let mut register_x = 1;
    let mut cycle = 1;

    let mut signal_sum = 0;

    for line in input.lines() {
        if (cycle + 20) % 40 == 0 {
            signal_sum += cycle * register_x;
            //print!("{}: sum={}\n", cycle, signal_sum);
        }
        cycle += 1;

        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["noop"] => {}
            ["addx", next_add] => {
                let next_add_value:i64 = next_add.parse().unwrap();
                if (cycle + 20) % 40 == 0 {
                    signal_sum += cycle * register_x;
                    //print!("{}: sum={}\n", cycle, signal_sum);
                }
                register_x += next_add_value;
                cycle += 1;
            }
            _ => {exit(1)}
        }
    }

    return signal_sum.to_string()
}

pub fn process_part2(input:&str) -> String {
    let mut sprite_position:i64 = 1;
    let mut pixel_position = 0;

    let mut screen:Vec<char> = Vec::new();

    for line in input.lines() {
        if (pixel_position - sprite_position).abs() <=1 {
            screen.push('#');
        } else {
            screen.push('.');
        }
        pixel_position += 1;
        if pixel_position == 40 {
            screen.push('\n');
            pixel_position = 0;
        }

        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["noop"] => {}
            ["addx", next_add] => {
                if (pixel_position - sprite_position).abs() <=1 {
                    screen.push('#');
                } else {
                    screen.push('.');
                }
                pixel_position += 1;
                if pixel_position == 40 {
                    screen.push('\n');
                    pixel_position = 0;
                }

                let next_add_value:i64 = next_add.parse().unwrap();
                sprite_position += next_add_value;
            }
            _ => {exit(1)}
        }
    }

    return screen.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, "13140");
    }

    #[test]
    fn test_part2() {
        let result = process_part2(TEST_INPUT);
        assert_eq!(result, TEST_RENDERING);
    }

    const TEST_RENDERING:&str =
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

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
