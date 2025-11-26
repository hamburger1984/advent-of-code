use core::panic;
use std::collections::{HashMap, HashSet};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    if let Some((warehouse, movements)) = input.split_once("\n\n") {
        let mut walls = Vec::new();
        let mut boxes = HashMap::new();
        let mut pos = 0;
        let mut w = warehouse.len();
        let mut h = 0;
        for (y, l) in warehouse.lines().enumerate() {
            h = y + 1;
            let l = l
                .replace(".", "..")
                .replace("#", "##")
                .replace("@", "@.")
                .replace("O", "[]");

            if w > l.len() {
                w = l.len();
            }
            println!("{}: {}", y, l);
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    walls.push(to_idx(x, y, w));
                } else if c == '.' {
                    continue;
                } else if c == '@' {
                    pos = to_idx(x, y, w);
                } else if c == '[' {
                    boxes.insert(to_idx(x, y, w), true);
                } else if c == ']' {
                    boxes.insert(to_idx(x, y, w), false);
                }
            }
        }
        println!("p:{}", pos);
        println!("w:{}", w);
        println!("h:{}", h);
        println!("walls: {:?}", walls);
        println!("boxes: {:?}", boxes);
        println!("{}", movements);

        println!("");
        for c in movements.chars() {
            if c == '\n' {
                continue;
                //break;
            }
            let (right_down, horizontal) = match c {
                '<' => (false, true),
                '^' => (false, false),
                '>' => (true, true),
                'v' => (true, false),
                _ => panic!("invalid movement"),
            };
            let amount = if horizontal { 1 } else { w };

            let next_pos = calc_next(pos, right_down, amount, 1);

            print!(
                "Move {} .. pos {} ({},{}) to {} ({},{})",
                c,
                pos,
                pos % w,
                (pos - pos % w) / w,
                next_pos,
                next_pos % w,
                (next_pos - next_pos % w) / w
            );

            // against wall
            if walls.contains(&next_pos) {
                println!(" .. wall");
                continue;
            }

            // no box
            if !boxes.contains_key(&next_pos) {
                pos = next_pos;
                println!("");
                //print(w, h, &boxes, &walls, pos);
                continue;
            }

            println!(" .. box");

            if horizontal {
                // easy cases: horizontal
                //  move
                //   -> @[][][].
                //   .[][][]@ <-
                //  no move
                //   -> @[][][]#
                //   #[][][]@ <-
                if !horizontal_box_push(&mut walls, &mut boxes, pos, w, h, right_down, next_pos) {
                    println!(" .. wall!");
                    continue;
                }
            } else {
                // hard cases: vertical
                // move                no move
                //  ..  v  ....  v     .#  v
                //  []  @  [][]  @     []  @
                //  []  []  []  []     []  []
                //  @   ..  @  [][]    @   #.
                //  ^       ^  ....    ^
                if !vertical_box_push(&mut walls, &mut boxes, pos, w, h, right_down, next_pos) {
                    println!(" .. wall!");
                    continue;
                }
            }

            pos = next_pos;
            print(w, h, &boxes, &walls, pos);
        }

        print(w, h, &boxes, &walls, pos);

        let total_box_coordinates = calc_total_gps(&boxes, w);
        return Ok(total_box_coordinates.to_string());
    }

    panic!("Could not find delimiting empty line");
}

fn vertical_box_push(
    walls: &Vec<usize>,
    mut boxes: &mut HashMap<usize, bool>,
    pos: usize,
    w: usize,
    h: usize,
    right_down: bool,
    next_pos: usize,
) -> bool {
    if let Some(v) = boxes.get(&next_pos) {
        // is next_pos already the start (aka '[') of a box? if not, align
        let mut to_check = vec![if *v { next_pos } else { next_pos - 1 }];

        let mut affected = Vec::new();

        while let Some(next) = to_check.pop() {
            print!("Checking {} ({},{})", next, next % w, (next - next % w) / w);
            if !boxes.contains_key(&next) {
                println!(" .. no box");
                continue;
            }
            if let Some(n) = boxes.get(&next) {
                print!(" .. box {} ", if *n { "start" } else { "end" });
                if !n {
                    println!("skip box end");
                    continue;
                }
            }
            println!("");
            affected.push(next);
            let next = calc_next(next, right_down, w, 1);

            if walls.contains(&next) || walls.contains(&(next + 1)) {
                //println!(
                //    "hit wall {} ({},{}) or {} ({},{}) - no move",
                //    next,
                //    next % w,
                //    (next - next % w) / w,
                //    next + 1,
                //    (next + 1) % w,
                //    ((next + 1) - (next + 1) % w) / w
                //);
                return false;
            }
            to_check.push(next - 1);
            to_check.push(next);
            to_check.push(next + 1);
        }

        affected.sort();
        if right_down {
            affected.reverse();
        }
        let multi = affected.len() > 1;

        for i in affected {
            move_box(i, &mut boxes, right_down, w, w);
            if multi {
                println!(".. step");
                print(w, h, &boxes, &walls, pos);
            }
        }
    }
    return true;
}

fn horizontal_box_push(
    walls: &Vec<usize>,
    mut boxes: &mut HashMap<usize, bool>,
    pos: usize,
    w: usize,
    h: usize,
    right_down: bool,
    next_pos: usize,
) -> bool {
    let mut box_pos = if right_down { next_pos } else { next_pos - 1 };
    let mut box_count = 1;
    while boxes.contains_key(&(calc_next(box_pos, right_down, 2, 1))) {
        box_count += 1;
        box_pos = calc_next(box_pos, right_down, 2, 1);
        //println!("Box at {}, now {} boxes", box_pos, box_count);
    }
    if right_down && walls.contains(&(box_pos + 2)) {
        return false;
    }
    if !right_down && walls.contains(&(box_pos - 2)) {
        return false;
    }
    let multi = box_count > 1;
    while box_count >= 1 {
        move_box(box_pos, &mut boxes, right_down, 1, w);
        if multi {
            print(w, h, &boxes, &walls, pos);
        }
        box_count -= 1;
        box_pos = calc_next(box_pos, !right_down, 2, 1);
    }
    true
}

fn calc_total_gps(boxes: &HashMap<usize, bool>, w: usize) -> usize {
    let mut total_box_coordinates = 0;
    for (b, v) in boxes.iter() {
        if !v {
            continue;
        }
        let x = b % w;
        let y = (b - x + 1) / w;
        let b_gps = 100 * y + x;
        //println!("({},{} ~ {}) {}", x, y, b, b_gps);

        total_box_coordinates += b_gps;
    }
    total_box_coordinates
}

fn move_box(pos: usize, boxes: &mut HashMap<usize, bool>, add: bool, amount: usize, w: usize) {
    if let Some(v) = boxes.remove(&pos) {
        let target = if add { pos + amount } else { pos - amount };
        //println!(
        //    "Move {},{} ({},{}) by ({},{}) to {}",
        //    pos,
        //    v,
        //    pos % w,
        //    (pos - pos % w) / w,
        //    add,
        //    amount,
        //    target
        //);
        if v {
            boxes.remove(&(pos + 1));
            boxes.insert(target, v);
            boxes.insert(target + 1, false);
        } else {
            boxes.remove(&(pos - 1));
            boxes.insert(target - 1, true);
            boxes.insert(target, v);
        }
    } else {
        panic!(
            "Failed to move box from {} by ({},{}) in {:?}",
            pos, add, amount, boxes
        );
    }
}

fn calc_next(start: usize, add: bool, amount: usize, count: usize) -> usize {
    if add {
        start + amount * count
    } else {
        start - amount * count
    }
}

fn print(w: usize, h: usize, boxes: &HashMap<usize, bool>, walls: &Vec<usize>, pos: usize) {
    for y in 0..h {
        for x in 0..w {
            let i = to_idx(x, y, w);

            let c = if i == pos {
                '@'
            } else if boxes.get(&i).is_some_and(|v| *v) {
                '['
            } else if boxes.get(&i).is_some_and(|v| !*v) {
                ']'
            } else if walls.contains(&i) {
                '#'
            } else if i == pos {
                '@'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!("");
    }
}

fn to_idx(x: usize, y: usize, w: usize) -> usize {
    y * w + x
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_simpler_process() -> miette::Result<()> {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        assert_eq!("618", process(input)?);
        Ok(())
    }

    //#[test]
    fn test_simple_process() -> miette::Result<()> {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!("1751", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!("9021", process(input)?);
        Ok(())
    }
}
