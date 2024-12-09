use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines.get(0).unwrap().len();
    let chars = lines.iter().flat_map(|l| l.chars()).collect::<Vec<char>>();

    let mut pos = chars.iter().position(|c| *c == '^').unwrap();
    let mut dir = GuardDir::Up;
    let mut visited = Vec::new();
    visited.push(pos);

    while let Some((new_pos, new_dir)) = step(pos, dir, width, &chars) {
        visited.push(new_pos);
        pos = new_pos;
        dir = new_dir;
    }

    Ok(visited.iter().unique().count().to_string())
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
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
