use std::usize::MAX;

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    println!("{}", input);
    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines.get(0).unwrap().len();
    let height = lines.len();
    let chars = lines.iter().flat_map(|l| l.chars()).collect::<Vec<char>>();
    let start_pos = chars.iter().position(|c| *c == '^').unwrap();
    println!("{}x{}", width, height);

    let visited = run_guard(start_pos, width, &chars, MAX).unwrap();

    // visited unique reversed
    // for each try:
    // * replace with #
    // * run up to width x height steps
    // * exited? -> yes, try next
    //           -> no, result += 1
    let mut loops = 0;
    for visited_pos in visited.iter().unique().rev() {
        if *visited_pos == start_pos {
            continue;
        }
        let mut new_obstacle = chars.to_vec();
        new_obstacle[*visited_pos] = '#';

        if let None = run_guard(start_pos, width, &new_obstacle, width * height) {
            loops += 1;
        }
    }

    Ok(loops.to_string())
}

fn run_guard(
    start_pos: usize,
    width: usize,
    chars: &Vec<char>,
    run_limit: usize,
) -> Option<Vec<usize>> {
    let mut pos = start_pos;
    let mut dir = GuardDir::Up;
    let mut visited = Vec::new();
    visited.push(pos);

    let mut steps = 0;

    while let Some((new_pos, new_dir)) = step(pos, dir, width, &chars) {
        steps += 1;
        if steps > run_limit {
            return None;
        }

        visited.push(new_pos);
        pos = new_pos;
        dir = new_dir;
    }

    Some(visited)
}

fn step(
    current_pos: usize,
    current_dir: GuardDir,
    width: usize,
    map: &Vec<char>,
) -> Option<(usize, GuardDir)> {
    if current_pos % width == 0 && current_dir == GuardDir::Left {
        return None;
    }
    if current_pos % width == width - 1 && current_dir == GuardDir::Right {
        return None;
    }
    if current_pos < width && current_dir == GuardDir::Up {
        return None;
    }

    let next_pos = match current_dir {
        GuardDir::Up => current_pos - width,
        GuardDir::Right => current_pos + 1,
        GuardDir::Down => current_pos + width,
        GuardDir::Left => current_pos - 1,
    };

    if next_pos >= map.len() {
        return None;
    }

    if map[next_pos] == '#' {
        return Some((current_pos, turn_right(current_dir)));
    }

    return Some((next_pos, current_dir));
}

fn turn_right(current_dir: GuardDir) -> GuardDir {
    return match current_dir {
        GuardDir::Up => GuardDir::Right,
        GuardDir::Right => GuardDir::Down,
        GuardDir::Down => GuardDir::Left,
        GuardDir::Left => GuardDir::Up,
    };
}

#[derive(Debug, PartialEq, Eq)]
enum GuardDir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
