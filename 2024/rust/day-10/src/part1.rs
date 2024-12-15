use std::collections::HashSet;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let width = input.split_once('\n').unwrap().0.len();

    let map = input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap() as u8)
        })
        .flatten()
        .collect::<Vec<u8>>();

    let height = map.len() / width;

    for y in 0..height {
        for x in 0..width {
            let index = to_index(x, y, width);
            if index == map.len() {
                panic!("nonono");
            }
            if map[index] == 9 {
                print!("\x1b[31m9\x1b[0m");
            } else if map[index] == 0 {
                print!("\x1b[32m0\x1b[0m");
            } else {
                print!("{}", map[index]);
            }
        }
        println!("");
    }

    let mut head_values_sum = 0;

    for (i, v) in map.iter().enumerate() {
        if *v != 0 {
            // not a trailhead
            continue;
        }

        let mut found: HashSet<usize> = HashSet::new();
        let mut todo: Vec<(usize, u8)> = Vec::new();
        todo.push((i, 0));

        while todo.len() > 0 {
            if let Some((curr, level)) = todo.pop() {
                if map[curr] == 9 {
                    found.insert(curr);
                    continue;
                }
                let next_level = level + 1;

                if (curr % width) > 0 && map[curr - 1] == next_level {
                    todo.push((curr - 1, next_level));
                }
                if (curr % width) < width - 1 && map[curr + 1] == next_level {
                    todo.push((curr + 1, next_level));
                }
                if curr > width && map[curr - width] == next_level {
                    todo.push((curr - width, next_level));
                }
                if curr < map.len() - width && map[curr + width] == next_level {
                    todo.push((curr + width, next_level));
                }
            }
        }

        println!("{}: {:?}", i, found);
        head_values_sum += found.len();
    }

    Ok(head_values_sum.to_string())
}

fn to_index(x: usize, y: usize, width: usize) -> usize {
    return x + y * width;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0123
1234
8765
9876";
        assert_eq!("1", process(input)?);

        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        assert_eq!("36", process(input)?);
        Ok(())
    }
}
