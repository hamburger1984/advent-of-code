use std::{collections::HashSet, process::exit};

pub fn process_part1(input: &str) -> String {
    let mut visited = HashSet::new();

    let (mut tx, mut ty, mut hx, mut hy) = (50, 50, 50, 50);

    visited.insert((tx, ty));

    for line in input.lines() {
        print!(" > head at {hx},{hy}, tail at {tx},{ty}\n");
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            ["L", steps] => {
                print!("Go {steps} left\n");
                for _ in 0..steps.parse().unwrap() {
                    hx -= 1;
                    (tx, ty) = calculate_tail(hx, hy, tx, ty);
                    visited.insert((tx, ty));
                }
            }
            ["R", steps] => {
                print!("Go {steps} right\n");
                for _ in 0..steps.parse().unwrap() {
                    hx += 1;
                    (tx, ty) = calculate_tail(hx, hy, tx, ty);
                    visited.insert((tx, ty));
                }
            }
            ["U", steps] => {
                print!("Go {steps} up\n");
                for _ in 0..steps.parse().unwrap() {
                    hy -= 1;
                    (tx, ty) = calculate_tail(hx, hy, tx, ty);
                    visited.insert((tx, ty));
                }
            }
            ["D", steps] => {
                print!("Go {steps} down\n");
                for _ in 0..steps.parse().unwrap() {
                    hy += 1;
                    (tx, ty) = calculate_tail(hx, hy, tx, ty);
                    visited.insert((tx, ty));
                }
            }
            _ => {
                exit(1);
            }
        }
    }

    print!(" > head at {hx},{hy}, tail at {tx},{ty}\n");

    return visited.len().to_string();
}

fn calculate_tail(hx: i32, hy: i32, tx: i32, ty: i32) -> (i32, i32) {
    if hx == tx && hy == ty {
        return (tx, ty);
    }

    if (hx - tx).abs() > 1 || (hy - ty).abs() > 1 {
        let mut tx2 = tx;
        let mut ty2 = ty;
        if hx > tx {
            tx2 += 1;
        }
        if hx < tx {
            tx2 -= 1;
        }

        if hy > ty {
            ty2 += 1;
        }
        if hy < ty {
            ty2 -= 1;
        }

        return (tx2, ty2);
    }

    return (tx, ty);
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
