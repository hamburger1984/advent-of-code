#![feature(int_roundings)]

pub fn process_part1(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines.first().unwrap().len();
    let height = lines.len();

    let mut map: Vec<u8> = Vec::new();
    let mut start = (0, 0);
    let mut goal = (0, 0);

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'a'..='z' => {
                    map.push(c as u8 - 'a' as u8);
                }
                'S' => {
                    map.push(0);
                    start = (x, y);
                }
                'E' => {
                    map.push('z' as u8 - 'a' as u8);
                    goal = (x, y);
                }
                _ => {
                    panic!("unexpected input");
                }
            }
        }
    }

    let path = a_star(
        start,
        goal,
        width,
        height,
        |x1, y1, x2, y2| {
            // Use the Manhattan distance as the heuristic
            (x1.abs_diff(x2) + y1.abs_diff(y2)).pow(2)
        },
        |x, y| {
            // Return the four neighboring cells
            let mut neighbors = Vec::new();

            // current level
            let current_level = map[to_index_xy(x, y, width)];

            // top neighbor present and reachable?
            if y > 0 && is_reachable(current_level, map[to_index_xy(x, y - 1, width)]) {
                neighbors.push((x, y - 1));
            }

            // bottom neighbor present and reachable?
            if y < height - 1 && is_reachable(current_level, map[to_index_xy(x, y + 1, width)]) {
                neighbors.push((x, y + 1));
            }

            // left neighbor present and reachable?
            if x > 0 && is_reachable(current_level, map[to_index_xy(x - 1, y, width)]) {
                neighbors.push((x - 1, y));
            }

            // right neighbor present and reachable?
            if x < width - 1 && is_reachable(current_level, map[to_index_xy(x + 1, y, width)]) {
                neighbors.push((x + 1, y));
            }

            neighbors
        },
    );

    // Print the path, if one was found
    if let Some(path) = path {
        print!("Arrived in {} steps\n", path.len());
        return path.len().to_string();
    } else {
        println!("No path found");
    }
    return "NO PATH".to_string();
}

use std::collections::BinaryHeap;

// Represents the state of a cell during the search
#[derive(Eq, PartialEq, Clone, Debug)]
struct State {
    index: usize,
    cost: usize,
    estimate: usize,
}

impl State {
    fn new(index: usize, cost: usize, estimate: usize) -> Self {
        Self {
            index,
            cost,
            estimate,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.cost + other.estimate).cmp(&(self.cost + self.estimate))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn to_index_pos(position: &(usize, usize), width: usize) -> usize {
    return (position.0 + position.1 * width) as usize;
}

fn to_index_xy(x: usize, y: usize, width: usize) -> usize {
    return (x + y * width) as usize;
}

fn to_position(index: usize, width: usize) -> (usize, usize) {
    let x = index % width;
    let y = index.div_floor(width);
    return (x, y);
}

fn is_reachable(current_level: u8, next_level: u8) -> bool {
    return current_level >= next_level || current_level as i16 == next_level as i16 - 1;
}

// A* search algorithm
fn a_star(
    start: (usize, usize),
    goal: (usize, usize),
    width: usize,
    height: usize,
    mut heuristic: impl FnMut(usize, usize, usize, usize) -> usize,
    mut successors: impl FnMut(usize, usize) -> Vec<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    let start_index = to_index_pos(&start, width);
    let goal_index = to_index_pos(&goal, width);

    let mut frontier = BinaryHeap::new();
    let mut cost_so_far = vec![usize::MAX; width * height];
    let mut came_from = vec![usize::MAX; width * height];
    let mut paths = Vec::new();

    frontier.push(State::new(
        start_index,
        0,
        heuristic(start.0, start.1, goal.0, goal.1),
    ));
    cost_so_far[start_index] = 0;

    // Continue searching while there are still unexplored states
    while let Some(State { index, cost, .. }) = frontier.pop() {
        let cell_position = to_position(index, width);

        // Check if we have reached the goal
        if index == goal_index {
            let mut map_dump = vec!['.'; width * height];

            // Reconstruct the path from the parent cells
            let mut path = vec![goal_index];
            let mut current = goal_index;

            map_dump[current] = 'E';
            while let Some(parent_index) = came_from.get(current) {
                if *parent_index == usize::MAX {
                    break;
                }
                if *parent_index == start_index {
                    map_dump[*parent_index] = 'S';
                    break;
                }

                let p1 = to_position(*parent_index, width);
                let p2 = to_position(current, width);
                if p1.0 < p2.0 {
                    map_dump[*parent_index] = '>';
                }
                if p1.0 > p2.0 {
                    map_dump[*parent_index] = '<';
                }
                if p1.1 < p2.1 {
                    map_dump[*parent_index] = 'v';
                }
                if p1.1 > p2.1 {
                    map_dump[*parent_index] = '^';
                }

                current = *parent_index;
                path.push(current);
            }

            for (i, c) in map_dump.iter().enumerate() {
                if i > 0 && i % width == 0 {
                    print!("\n");
                }
                print!("{}", c);
            }
            print!("\n");

            for _ in 0..width {
                print!("-");
            }
            print!("\nCOST {}\n", cost);

            paths.push((cost, path.clone()));
            continue;
        }

        // Get the successors of the current cell
        for next in successors(cell_position.0, cell_position.1) {
            let next_index = to_index_pos(&next, width);

            if next_index == start_index {
                continue;
            }

            // Calculate the cost of reaching this cell
            let new_cost = cost + 1;

            let current_cost = cost_so_far[next_index];

            if new_cost < current_cost {
                cost_so_far[next_index] = new_cost;
                came_from[next_index] = index;
                frontier.push(State::new(
                    next_index,
                    new_cost,
                    heuristic(next.0, next.1, goal.0, goal.1),
                ));
            }
        }
    }

    if paths.len() == 0 {
        return None;
    }

    paths.sort_by(|a, b| a.0.cmp(&b.0));

    let best_path = paths.first().unwrap().1.clone();

    // Return the path in reverse order
    return Some(
        best_path
            .into_iter()
            .rev()
            .map(|i| to_position(i, width))
            .collect(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process_part1(TESTINPUT);
        assert_eq!(result, "31");
    }

    #[test]
    fn test_position_index_conversions() {
        assert_eq!(1, to_index_xy(1, 0, 8));
        assert_eq!(7, to_index_xy(7, 0, 8));
        assert_eq!(8 * 7, to_index_xy(0, 7, 8));
    }

    const TESTINPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
}
