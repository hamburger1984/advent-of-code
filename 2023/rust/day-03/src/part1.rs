use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut symbols_or_numbers: Vec<SymbolOrNumber> = Vec::new();

    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        while x < line.len() {
            let sub_line = &line[x..];
            if (is_symbol(sub_line)) {
                symbols_or_numbers.push(SymbolOrNumber::Symbol { c: sub_line.chars().next().unwrap(), x: x, y: y })
            }
            match (is_symbol_or_number(x, y, sub_line)) {
                None => { x += 1; }
                Some(s) => {}
            }
        }
        y += 1;
    }
    //let result = input.lines().filter_map(symbols_or_numbers).collect::<Vec<SymbolOrNumber>>();

    return Ok("asdf".to_string());
}

fn is_symbol(p0: &str) -> bool {
    todo!()
}

fn is_symbol_or_number(x: usize, y: usize, line: &str) -> Option<SymbolOrNumber> {
    todo!()
}


enum SymbolOrNumber {
    Symbol { c: char, x: usize, y: usize },
    Number { value: u32, x: u32, y: u32, len: u32 },
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
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
