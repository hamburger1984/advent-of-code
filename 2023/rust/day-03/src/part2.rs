use std::collections::{HashMap, HashSet};
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let symbols_or_numbers = find_symbols_and_numbers(input);

    //println!("{}", input);
    let numbers = find_gear_ratios(symbols_or_numbers);

    return Ok(numbers.iter().sum::<u32>().to_string());
}

fn find_gear_ratios(symbols_or_numbers: Vec<SymbolOrNumber>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    let mut gear_positions: HashSet<Position> = HashSet::new();
    let mut number_neighborhoods: HashMap<Position, Vec<u32>> = HashMap::new();

    for e in symbols_or_numbers {
        match e {
            SymbolOrNumber::Number { pos, len, value } => {
                let start_x = if pos.x > 0 { pos.x - 1 } else { pos.x };
                let start_y = if pos.y > 0 { pos.y - 1 } else { pos.y };
                for ny in start_y..=(pos.y + 1) {
                    for nx in start_x..=(pos.x + len) {
                        let neighbor = Position { x: nx, y: ny };
                        if number_neighborhoods.contains_key(&neighbor) {
                            number_neighborhoods.get_mut(&neighbor).unwrap().push(value);
                        } else {
                            number_neighborhoods.insert(Position { x: nx, y: ny }, vec![value]);
                        }
                    }
                }
            }
            SymbolOrNumber::Symbol { pos, c } => {
                if c == '*' {
                    gear_positions.insert(Position { x: pos.x, y: pos.y });
                }
            }
        }
    }

    for gp in gear_positions {
        match number_neighborhoods.get(&gp) {
            None => {}
            Some(numbers) => {
                if numbers.len() == 2 {
                    result.push(numbers.get(0).unwrap() * numbers.get(1).unwrap());
                }
            }
        }
    }

    result
}

fn find_symbols_and_numbers(input: &str) -> Vec<SymbolOrNumber> {
    let mut result: Vec<SymbolOrNumber> = Vec::new();
    let mut y = 0;
    for line in input.lines() {
        let mut part_id = 0;
        let mut part_id_start = 0;
        let mut x = 0;
        for c in line.chars() {
            if c == '.' {
                if part_id > 0 {
                    result.push(SymbolOrNumber::Number { value: part_id, pos: Position { x: part_id_start, y }, len: x - part_id_start });
                    part_id = 0;
                }
            } else if c.is_digit(10) {
                if part_id == 0 { part_id_start = x; } else { part_id *= 10; }
                part_id += c.to_digit(10).expect("When c is a digit, it is a digit");
            } else {
                if part_id > 0 {
                    result.push(SymbolOrNumber::Number { value: part_id, pos: Position { x: part_id_start, y }, len: x - part_id_start });
                    part_id = 0;
                }
                result.push(SymbolOrNumber::Symbol { c, pos: Position { x, y } })
            }
            x += 1;
        }
        if part_id > 0 {
            result.push(SymbolOrNumber::Number { value: part_id, pos: Position { x: part_id_start, y }, len: x - part_id_start });
        }

        y += 1;
    }
    result
}

#[derive(Debug)]
enum SymbolOrNumber {
    Symbol { c: char, pos: Position },
    Number { value: u32, pos: Position, len: u32 },
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}