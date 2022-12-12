use std::{collections::HashSet, process::exit};

pub fn process_part1(input: &str) -> String {
    let mut visited = HashSet::new();

    let (mut tx, mut ty, mut hx, mut hy) = (0, 0, 0, 0);

    visited.insert((tx, ty));

    for line in input.lines() {
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            ["L", steps] => {
                print!("Go {steps} left\n");
                for _ in 0..steps.parse().unwrap() {
                    hx -= 1;
                }
            }
            ["R", steps] => {
                print!("Go {steps} right\n");
                for _ in 0..steps.parse().unwrap() {
                    hx += 1;
                }
            }
            ["U", steps] => {
                print!("Go {steps} up\n");
                for _ in 0..steps.parse().unwrap() {
                    hy -= 1;
                }
            }
            ["D", steps] => {
                print!("Go {steps} down\n");
                for _ in 0..steps.parse().unwrap() {
                    hy += 1;
                }
            }
            _ => {
                exit(1);
            }
        }
        print!(" > head at {hx},{hy}\n")
    }
    return 1.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, "13");
    }

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
}
