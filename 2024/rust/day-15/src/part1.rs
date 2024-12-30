use core::panic;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    if let Some((warehouse, movements)) = input.split_once("\n\n") {
        let mut walls = Vec::new();
        let mut boxes = Vec::new();
        let mut pos = 0;
        let mut w = warehouse.len();
        let mut h = 0;
        for (y, l) in warehouse.lines().enumerate() {
            h = y + 1;
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
                } else if c == 'O' {
                    boxes.push(to_idx(x, y, w));
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
            }
            let (add, offset) = match c {
                '<' => (false, 1),
                '^' => (false, w),
                '>' => (true, 1),
                'v' => (true, w),
                _ => panic!("invalid movement"),
            };

            let next_pos = calc_next(pos, add, offset, 1); //if dir { pos + offset } else { pos - offset };

            //println!("Move {} .. pos {}", c, pos);

            // against wall
            if walls.contains(&next_pos) {
                continue;
            }
            // no box
            if !boxes.contains(&next_pos) {
                pos = next_pos;
                //print(w, h, &boxes, &walls, pos);
                continue;
            }

            // how many boxes?
            let mut box_line = 0;
            //print!("Box {}: ", next_pos);
            while boxes.contains(&(calc_next(pos, add, offset, 1 + box_line))) {
                //print!("O");
                box_line += 1;
            }
            //println!("");

            // boxes stuck on wall?
            if walls.contains(&(calc_next(pos, add, offset, 1 + box_line))) {
                continue;
            }
            while box_line > 0 {
                let end = calc_next(pos, add, offset, box_line);
                let moved_end = calc_next(pos, add, offset, 1 + box_line);
                //println!("Moving from {} to {} .. all {:?}", end, moved_end, boxes);
                let box_index = boxes.iter().position(|i| *i == end).unwrap();
                boxes.remove(box_index);
                boxes.push(moved_end);
                box_line -= 1;
            }
            pos = next_pos;
            //print(w, h, &boxes, &walls, pos);
        }
        let mut total_box_coordinates = 0;
        for b in &boxes {
            let x = b % w;
            let y = (b - x + 1) / w;
            let b_gps = 100 * y + x;
            //println!("({},{} ~ {}) {}", x, y, b, b_gps);

            total_box_coordinates += b_gps;
        }

        print(w, h, &boxes, &walls, pos);

        return Ok(total_box_coordinates.to_string());
    }

    panic!("Could not find delimiting empty line");
}

fn calc_next(start: usize, add: bool, amount: usize, count: usize) -> usize {
    if add {
        start + amount * count
    } else {
        start - amount * count
    }
}

fn print(w: usize, h: usize, boxes: &Vec<usize>, walls: &Vec<usize>, pos: usize) {
    for y in 0..h {
        for x in 0..w {
            let i = to_idx(x, y, w);

            let c = if boxes.contains(&i) {
                'O'
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

    #[test]
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
        assert_eq!("2028", process(input)?);
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
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
        assert_eq!("10092", process(input)?);
        Ok(())
    }
}
