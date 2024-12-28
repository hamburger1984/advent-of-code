use std::iter;

use regex::Regex;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    return process_impl(input, (101, 103));
}

#[tracing::instrument]
pub fn process_impl(input: &str, area: (i32, i32)) -> miette::Result<String, AocError> {
    let seconds = 100;
    println!("Area {},{} Seconds {}", area.0, area.1, seconds);

    if let Ok(robot_regex) = Regex::new(r"p=(-?\d+),(-?\d+)\sv=(-?\d+),(-?\d+)") {
        let robots = input
            .lines()
            .filter(|l| l.len() > 0)
            .map(|l| {
                let cap = robot_regex.captures(l).unwrap();
                return (
                    cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                );
            })
            .collect::<Vec<(i32, i32, i32, i32)>>();

        let moved_robots = robots
            .iter()
            .map(|r| {
                let delta = (r.2 * seconds, r.3 * seconds);
                let moved = (
                    (r.0 + delta.0 + seconds * area.0) % area.0,
                    (r.1 + delta.1 + seconds * area.1) % area.1,
                );
                println!(
                    "{},{} -- {},{} -> {},{}",
                    r.0, r.1, delta.0, delta.1, moved.0, moved.1
                );

                return moved;
            })
            .collect::<Vec<(i32, i32)>>();

        let mut quadrants = vec![0, 0, 0, 0];

        let mut map = iter::repeat(0)
            .take((area.0 * area.1) as usize)
            .collect::<Vec<u16>>();

        for r in moved_robots {
            map[to_idx(r, area.0)] += 1;
            if let Some(q) = get_quad(r, area) {
                quadrants[q] += 1;
            }
        }

        for y in 0..area.1 {
            for x in 0..area.0 {
                let n = map[to_idx((x, y), area.0)];
                if get_quad((x, y), area) == None {
                    print!(" ");
                    continue;
                }
                if n == 0 {
                    print!(".");
                } else {
                    print!("{}", n);
                }
            }
            println!("");
        }
        return Ok((quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]).to_string());
    }

    panic!("Failed to initialize regex");
}

fn to_idx(loc: (i32, i32), w: i32) -> usize {
    (loc.0 + loc.1 * w) as usize
}

fn get_quad(r: (i32, i32), area: (i32, i32)) -> Option<usize> {
    let left_end = area.0 / 2;
    let right_start = area.0 - left_end;
    let top_end = area.1 / 2;
    let bottom_start = area.1 - top_end;

    if r.0 < left_end {
        if r.1 < top_end {
            return Some(0);
        }
        if r.1 >= bottom_start {
            return Some(1);
        }
    }
    if r.0 >= right_start {
        if r.1 < top_end {
            return Some(2);
        }
        if r.1 >= bottom_start {
            return Some(3);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process_impl(input, (11, 7))?);
        Ok(())
    }
}
