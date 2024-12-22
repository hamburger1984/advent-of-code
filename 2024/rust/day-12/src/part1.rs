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

        // edges:
        //  for all tiles
        //  * is the left tile here? -> 0, else -> 1
        //  * is the top tile here? -> 0, else -> 1
        //  ...

        let mut edges = 0;
        for t in tiles.iter() {
            if *t % width == 0 || !tiles.contains(&(t - 1)) {
                edges += 1;
            }
            if *t % width == width - 1 || !tiles.contains(&(t + 1)) {
                edges += 1;
            }
            if *t < width || !tiles.contains(&(t - width)) {
                edges += 1;
            }
            if *t + width >= map.len() || !tiles.contains(&(t + width)) {
                edges += 1;
            }
        }

        sum += tiles.len() * edges;
        println!(
            "{}: {} tiles, {} edges --- sum {}",
            c,
            tiles.len(),
            edges,
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
        assert_eq!("140", process(input)?);

        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
        assert_eq!("772", process(input)?);

        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
