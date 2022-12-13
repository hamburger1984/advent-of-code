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
                    (tx, ty) = calculate_tail((hx, hy), (tx, ty));
                    visited.insert((tx, ty));
                }
            }
            ["R", steps] => {
                print!("Go {steps} right\n");
                for _ in 0..steps.parse().unwrap() {
                    hx += 1;
                    (tx, ty) = calculate_tail((hx, hy), (tx, ty));
                    visited.insert((tx, ty));
                }
            }
            ["U", steps] => {
                print!("Go {steps} up\n");
                for _ in 0..steps.parse().unwrap() {
                    hy -= 1;
                    (tx, ty) = calculate_tail((hx, hy), (tx, ty));
                    visited.insert((tx, ty));
                }
            }
            ["D", steps] => {
                print!("Go {steps} down\n");
                for _ in 0..steps.parse().unwrap() {
                    hy += 1;
                    (tx, ty) = calculate_tail((hx, hy), (tx, ty));
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

pub fn process_part2(input: &str) -> String {
    let mut visited = HashSet::new();

    let mut rope = vec![(50, 50); 10];

    visited.insert(rope[0]);

    for line in input.lines() {
        print!(
            " > head at {:?}, tail at {:?}\n",
            rope.first().unwrap(),
            rope.last().unwrap()
        );
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            ["L", steps] => {
                print!("Go {steps} left\n");
                for _ in 0..steps.parse().unwrap() {
                    rope[0] = (rope[0].0 - 1, rope[0].1);
                    update_rope(& mut rope);
                    visited.insert(*rope.last().unwrap());
                }
            }
            ["R", steps] => {
                print!("Go {steps} right\n");
                for _ in 0..steps.parse().unwrap() {
                    rope[0] = (rope[0].0 + 1, rope[0].1);
                    update_rope(& mut rope);
                    visited.insert(*rope.last().unwrap());
                }
            }
            ["U", steps] => {
                print!("Go {steps} up\n");
                for _ in 0..steps.parse().unwrap() {
                    rope[0] = (rope[0].0, rope[0].1 - 1);
                    update_rope(& mut rope);
                    visited.insert(*rope.last().unwrap());
                }
            }
            ["D", steps] => {
                print!("Go {steps} down\n");
                for _ in 0..steps.parse().unwrap() {
                    rope[0] = (rope[0].0, rope[0].1 + 1);
                    update_rope(& mut rope);
                    visited.insert(*rope.last().unwrap());
                }
            }
            _ => {
                exit(1);
            }
        }
    }

    print!(
        " > head at {:?}, tail at {:?}\n",
        rope.first().unwrap(),
        rope.last().unwrap()
    );

    return visited.len().to_string();
}

fn update_rope(rope: &mut Vec<(i32, i32)>) {
    for i in 1..rope.len() {
        rope[i] = calculate_tail(rope[i - 1], rope[i]);
    }
}

fn calculate_tail(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    if h == t {
        return t;
    }

    if (h.0 - t.0).abs() > 1 || (h.1 - t.1).abs() > 1 {
        let mut tx2 = t.0;
        let mut ty2 = t.1;
        if h.0 > t.0 {
            tx2 += 1;
        }
        if h.0 < t.0 {
            tx2 -= 1;
        }

        if h.1 > t.1 {
            ty2 += 1;
        }
        if h.1 < t.1 {
            ty2 -= 1;
        }

        return (tx2, ty2);
    }

    return t;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process_part1(TEST_INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn test_part2() {
        let result = process_part2(TEST_INPUT2);
        assert_eq!(result, "36");
    }

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
}
