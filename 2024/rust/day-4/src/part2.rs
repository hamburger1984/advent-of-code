use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines = input.lines().collect::<Vec<&str>>();
    let line_len = lines.get(0).unwrap().len();
    let lines_count = lines.len();
    let chars = lines.iter().flat_map(|l| l.chars()).collect::<Vec<char>>();
    let a_indices = chars
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == 'A')
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();

    println!("{}", input);

    let mut found = 0;

    for i in a_indices {
        let col = i % line_len;
        let row = i / line_len;

        if col == 0 || col == line_len - 1 || row == 0 || row == lines_count - 1 {
            println!(" at edge, skip! {} ({},{})", i, col, row);
            continue;
        }

        if ((chars.get(i - 1 - line_len).unwrap() == &'M'
            && chars.get(i + 1 + line_len).unwrap() == &'S')
            || (chars.get(i - 1 - line_len).unwrap() == &'S'
                && chars.get(i + 1 + line_len).unwrap() == &'M'))
            && ((chars.get(i - 1 + line_len).unwrap() == &'M'
                && chars.get(i + 1 - line_len).unwrap() == &'S')
                || (chars.get(i - 1 + line_len).unwrap() == &'S'
                    && chars.get(i + 1 - line_len).unwrap() == &'M'))
        {
            println!("X-MAS {} ({},{})", i, col, row);
            found += 1;
        }
    }

    Ok(found.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
