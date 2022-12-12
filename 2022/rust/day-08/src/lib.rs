
pub fn process_part1(input: &str) -> String {
    let mut cells = Vec::new();

    let lines = input.lines().collect::<Vec<&str>>();

    let cols = lines[0].chars().count();
    let rows = lines.len();

    for line in lines {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .for_each(|v| {
                cells.push(v);
            })
    }

    let edge = 2 * cols + 2 * (rows - 2);
    let mut inner = 0;

    for y in 1..(rows - 1) {
        for x in 1..(cols - 1) {
            if is_visible(x, y, cols, rows, &cells) {
                inner = inner + 1;
            }
        }
    }

    dbg!(edge, inner);

    return (edge + inner).to_string();
}

fn is_visible(x: usize, y: usize, cols: usize, rows: usize, cells: &Vec<u8>) -> bool {
    let tree = cells[x + y * rows];

    let mut visible = true;
    for x_l in 0..x {
        if cells[x_l + y * rows] >= tree {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    visible = true;
    for x_r in (x + 1)..cols {
        if cells[x_r + y * rows] >= tree {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    visible = true;
    for y_t in 0..y {
        if cells[x + y_t * rows] >= tree {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    visible = true;
    for y_b in (y + 1)..rows {
        if cells[x + y_b * rows] >= tree {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process_part1(TESTINPUT);
        assert_eq!(result, "21");
    }

    const TESTINPUT: &str = "30373
25512
65332
33549
35390";
}
