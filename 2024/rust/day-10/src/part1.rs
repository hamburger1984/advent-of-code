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
                panic!("what?!")
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
        if *v == 0 {
            let (x, y) = from_index(i, width);
            print!("{} ({},{}) =", i, x, y);
            let head_value = trail_head_value(width, &map, 0, i, Vec::new());
            print!("> {:?} ", head_value);
            println!(
                " .. {} -> {}",
                head_values_sum,
                head_values_sum + head_value.len()
            );
            head_values_sum += head_value.len();
        }
    }
    Ok(head_values_sum.to_string())
}

fn to_index(x: usize, y: usize, width: usize) -> usize {
    return x + y * width;
}

fn from_index(index: usize, width: usize) -> (usize, usize) {
    return (index % width, index / width);
}

fn trail_head_value(
    width: usize,
    map: &Vec<u8>,
    curr_level: u8,
    index: usize,
    found: Vec<usize>,
) -> Vec<usize> {
    if curr_level > 9 {
        return found;
    }

    if map[index] == 9 {
        if found.contains(&index) {
            return found;
        }
        let mut res = found.clone();
        res.push(index);
        return res;
    }

    let next_level = curr_level + 1;
    let mut result = found.clone();

    if index > 0 && map[index - 1] == next_level {
        result = trail_head_value(width, map, next_level, index - 1, result);
    }
    if index < map.len() - 1 && map[index + 1] == next_level {
        result = trail_head_value(width, map, next_level, index + 1, result);
    }
    if index > width && map[index - width] == next_level {
        result = trail_head_value(width, map, next_level, index - width, result);
    }
    if index < map.len() - width && map[index + width] == next_level {
        result = trail_head_value(width, map, next_level, index + width, result);
    }

    return result;
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
