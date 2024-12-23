use std::collections::HashSet;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let width = input.split_once('\n').unwrap().0.len();
    let mut map = input
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| l.chars())
        .flatten()
        .collect::<Vec<char>>();
    let height = map.len() / width;

    for y in 0..height {
        for x in 0..width {
            print!("{}", map[x + y * width]);
        }
        println!("");
    }

    let mut sum = 0;

    for i in 0..map.len() {
        let c = map[i];
        if c == '.' {
            continue;
        }

        let mut todo = Vec::new();
        todo.push(i);

        let mut tiles = HashSet::new();
        tiles.insert(i);

        while todo.len() > 0 {
            let i = todo.pop().unwrap();
            if map[i] != c {
                continue;
            }
            map[i] = '.';

            if i % width > 0 && map[i - 1] == c {
                todo.push(i - 1);
                tiles.insert(i - 1);
            }
            if i % width < width - 1 && map[i + 1] == c {
                todo.push(i + 1);
                tiles.insert(i + 1);
            }
            if i >= width && map[i - width] == c {
                todo.push(i - width);
                tiles.insert(i - width);
            }
            if i + width < map.len() && map[i + width] == c {
                todo.push(i + width);
                tiles.insert(i + width);
            }
        }

        let mut left_visited = HashSet::new();
        let mut right_visited = HashSet::new();
        let mut top_visited = HashSet::new();
        let mut bottom_visited = HashSet::new();
        let mut sides = 0;
        for t in tiles.iter() {
            // left
            if !left_visited.contains(t) && is_left_edge(*t, &tiles, width) {
                sides += 1;
                println!("< {} ({},{}) -> {}", *t, *t % width, *t / width, sides);

                let mut temp = *t;

                // search up
                while tiles.contains(&temp) && is_left_edge(temp, &tiles, width) {
                    left_visited.insert(temp);
                    if temp < width {
                        break;
                    }
                    temp -= width;
                }

                temp = *t;
                // search down
                while tiles.contains(&temp) && is_left_edge(temp, &tiles, width) {
                    left_visited.insert(temp);
                    if temp + width >= map.len() {
                        break;
                    }
                    temp += width;
                }
            }
            println!("Right? {} ({},{})", *t, *t % width, *t / width,);
            // right
            if !right_visited.contains(t) && is_right_edge(*t, &tiles, width) {
                sides += 1;
                println!("> {} ({},{}) -> {}", *t, *t % width, *t / width, sides);

                let mut temp = *t;

                // search up
                while tiles.contains(&temp) && is_right_edge(temp, &tiles, width) {
                    right_visited.insert(temp);
                    if temp < width {
                        break;
                    }
                    temp -= width;
                }

                temp = *t;
                // search down
                while tiles.contains(&temp) && is_right_edge(temp, &tiles, width) {
                    right_visited.insert(temp);
                    if temp + width >= map.len() {
                        break;
                    }
                    temp += width;
                }
            }
            // up
            if !top_visited.contains(t) && is_top_edge(*t, &tiles, width) {
                sides += 1;
                println!("^ {} ({},{}) -> {}", *t, *t % width, *t / width, sides);

                let mut temp = *t;

                // search left
                while tiles.contains(&temp) && is_top_edge(temp, &tiles, width) {
                    top_visited.insert(temp);
                    if temp % width == 0 {
                        break;
                    }
                    temp -= 1;
                }

                temp = *t;
                // search right
                while tiles.contains(&temp) && is_top_edge(temp, &tiles, width) {
                    top_visited.insert(temp);
                    if temp + 1 % width == 0 {
                        break;
                    }
                    temp += 1;
                }
            }
            // down
            if !bottom_visited.contains(t) && is_bottom_edge(*t, &tiles, width, map.len()) {
                sides += 1;
                println!("v {} ({},{}) -> {}", *t, *t % width, *t / width, sides);

                let mut temp = *t;

                // search left
                while tiles.contains(&temp) && is_bottom_edge(temp, &tiles, width, map.len()) {
                    bottom_visited.insert(temp);
                    if temp % width == 0 {
                        break;
                    }
                    temp -= 1;
                }

                temp = *t;
                // search right
                while tiles.contains(&temp) && is_bottom_edge(temp, &tiles, width, map.len()) {
                    bottom_visited.insert(temp);
                    if temp + 1 % width == 0 {
                        break;
                    }
                    temp += 1;
                }
            }
        }

        sum += tiles.len() * sides;
        println!(
            "{}: {} tiles, {} sides = {} --- sum {}",
            c,
            tiles.len(),
            sides,
            tiles.len() * sides,
            sum
        );

        for y in 0..height {
            for x in 0..width {
                print!("{}", map[x + y * width]);
            }
            println!("");
        }
    }

    Ok(sum.to_string())
}

fn is_left_edge(i: usize, tiles: &HashSet<usize>, width: usize) -> bool {
    i % width == 0 || !tiles.contains(&(i - 1))
}

fn is_right_edge(i: usize, tiles: &HashSet<usize>, width: usize) -> bool {
    i % width == width - 1 || !tiles.contains(&(i + 1))
}

fn is_top_edge(i: usize, tiles: &HashSet<usize>, width: usize) -> bool {
    i < width || !tiles.contains(&(i - width))
}

fn is_bottom_edge(i: usize, tiles: &HashSet<usize>, width: usize, map_len: usize) -> bool {
    i + width >= map_len || !tiles.contains(&(i + width))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "AAAA
BBCD
BBCC
EEEC
";
        assert_eq!("80", process(input)?);

        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
        assert_eq!("436", process(input)?);

        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
        assert_eq!("236", process(input)?);

        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
        assert_eq!("368", process(input)?);
        Ok(())
    }
}
