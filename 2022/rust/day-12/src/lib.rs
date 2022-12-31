pub fn process_part1(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines.first().unwrap().len();
    let height = lines.len();
    print!("{} x {}\n", width, height);

    let mut map: Vec<u8> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut goal: (usize, usize) = (0, 0);

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

    for y in 0..height {
        print!("{:>2}: ", y);
        for x in 0..width {
            if start == (x, y) {
                print!(">{:>2}<", map[y * width + x]);
            } else if goal == (x, y) {
                print!("<{:>2}>", map[y * width + x]);
            } else {
                print!(" {:>2} ", map[y * width + x]);
            }
        }
        print!("\n");
    }

    let mut open = vec![start];
    let mut closed: Vec<(usize, usize)> = Vec::new();

    while let current = open.pop {}

    // --- chat gpt generated:
    let start = Cell::new(0, 0);
    let goal = Cell::new(4, 4);

    let path = a_star(
        start,
        goal,
        |current, goal| {
            // Use the Manhattan distance as the heuristic
            (goal.x - current.x).abs() as f64 + (goal.y - current.y).abs() as f64
        },
        |current| {
            // Return the four neighboring cells
            let mut neighbors = Vec::new();
            neighbors.push(Cell::new(current.x, current.y - 1));
            neighbors.push(Cell::new(current.x, current.y + 1));
            neighbors.push(Cell::new(current.x - 1, current.y));
            neighbors.push(Cell::new(current.x + 1, current.y));
            neighbors
        },
    );

    // Print the path, if one was found
    if let Some(path) = path {
        for cell in path {
            println!("({}, {})", cell.x, cell.y);
        }
    } else {
        println!("No path found");
    }
    // --- end of generated

    return "nope".to_string();
}

// --- chat gpt generated:
use std::collections::{BinaryHeap, HashMap};

// Represents a single cell in the grid
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// Represents the state of a cell during the search
#[derive(Eq, PartialEq, Clone, Debug)]
struct State {
    cell: Cell,
    cost: f64,
    estimate: f64,
}

impl State {
    fn new(cell: Cell, cost: f64, estimate: f64) -> Self {
        Self {
            cell,
            cost,
            estimate,
        }
    }
}

// The priority queue used by A* needs to know how to order its elements
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Invert the ordering so the priority queue becomes a min-heap
        other
            .cost
            .add(other.estimate)
            .partial_cmp(&self.cost.add(self.estimate))
            .unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// A* search algorithm
fn a_star(
    start: Cell,
    goal: Cell,
    mut heuristic: impl FnMut(Cell, Cell) -> f64,
    mut successors: impl FnMut(Cell) -> Vec<Cell>,
) -> Option<Vec<Cell>> {
    // Priority queue of unexplored states, ordered by their total cost + estimate
    let mut frontier = BinaryHeap::new();

    // Map of explored cells and their corresponding cost
    let mut cost_so_far = HashMap::new();

    // Map of cell to its parent cell, used to reconstruct the path at the end
    let mut came_from:HashMap<Cell, Cell> = HashMap::new();

    // Start the search with the initial state
    frontier.push(State::new(start, 0.0, heuristic(start, goal)));
    cost_so_far.insert(start, 0.0);

    // Continue searching while there are still unexplored states
    while let Some(State { cell, cost, .. }) = frontier.pop() {
        // Check if we have reached the goal
        if cell == goal {
            // Reconstruct the path from the parent cells
            let mut path = vec![goal];
            let mut current = goal;
            while let Some(came_from_cell) = came_from.get(&current) {
                current = came_from_cell.clone();
                path.push(current.clone());
            }

            // Return the path in reverse order
            return Some(path.into_iter().rev().collect());
        }

        // Get the successors of the current cell
        // Get the successors of the current cell
        for next in successors(cell) {
            // Calculate the cost of reaching this cell
            let new_cost = cost + 1.0;

            // Check if we have found a cheaper way to reach this cell
            if let Some(current_cost) = cost_so_far.get(&next) {
                if new_cost < *current_cost {
                    cost_so_far.insert(next.clone(), new_cost);
                    came_from.insert(next.clone(), cell.clone());
                    let estimate = heuristic(next, goal);
                    frontier.push(State::new(next, new_cost, estimate));
                }
            } else {
                // This cell has not been explored yet
                cost_so_far.insert(next.clone(), new_cost);
                came_from.insert(next.clone(), cell.clone());
                let estimate = heuristic(next, goal);
                frontier.push(State::new(next, new_cost, estimate));
            }
        }
    }

    // Return None if no path was found
    None
}

// --- end of generated

fn manhattan_distance(x1: u16, y1: u16, x2: u16, y2: u16) -> u16 {
    return x1.abs_diff(x2) + y1.abs_diff(y2);
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
    fn test_manhattan() {
        assert_eq!(manhattan_distance(0, 0, 10, 10), 20);
        assert_eq!(manhattan_distance(10, 10, 10, 10), 0);
        assert_eq!(manhattan_distance(10, 10, 0, 0), 20);
    }

    const TESTINPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
}
