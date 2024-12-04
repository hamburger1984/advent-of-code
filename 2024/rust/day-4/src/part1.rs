use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let lines = input.lines().collect::<Vec<&str>>();
    let line_len = lines.get(0).unwrap().len();
    let lines_count = lines.len();
    let chars = lines.iter().flat_map(|l| l.chars()).collect::<Vec<char>>();
    let x_indices = chars
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == 'X')
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();

    println!("{}", input);

    let mut found = 0;

    for i in x_indices {
        let col = i % line_len;
        let row = i / line_len;
        //println!("{} ({},{})", i, col, row);
        if col < line_len - 3
            && chars.get(i + 1).unwrap() == &'M'
            && chars.get(i + 2).unwrap() == &'A'
            && chars.get(i + 3).unwrap() == &'S'
        {
            println!("XMAS lr {} ({},{})", i, col, row);
            found += 1;
        }
        if col >= 3
            && chars.get(i - 1).unwrap() == &'M'
            && chars.get(i - 2).unwrap() == &'A'
            && chars.get(i - 3).unwrap() == &'S'
        {
            println!("XMAS rl {} ({},{})", i, col, row);
            found += 1;
        }
        if row < lines_count - 3
            && chars.get(i + 1 * line_len).unwrap() == &'M'
            && chars.get(i + 2 * line_len).unwrap() == &'A'
            && chars.get(i + 3 * line_len).unwrap() == &'S'
        {
            println!("XMAS dn {} ({},{})", i, col, row);
            found += 1;
        }
        if row >= 3
            && chars.get(i - 1 * line_len).unwrap() == &'M'
            && chars.get(i - 2 * line_len).unwrap() == &'A'
            && chars.get(i - 3 * line_len).unwrap() == &'S'
        {
            println!("XMAS up {} ({},{})", i, col, row);
            found += 1;
        }
        if row >= 3
            && col >= 3
            && chars.get(i - 1 - 1 * line_len).unwrap() == &'M'
            && chars.get(i - 2 - 2 * line_len).unwrap() == &'A'
            && chars.get(i - 3 - 3 * line_len).unwrap() == &'S'
        {
            println!("XMAS ul {} ({},{})", i, col, row);
            found += 1;
        }
        if row >= 3
            && col < line_len - 3
            && chars.get(i + 1 - 1 * line_len).unwrap() == &'M'
            && chars.get(i + 2 - 2 * line_len).unwrap() == &'A'
            && chars.get(i + 3 - 3 * line_len).unwrap() == &'S'
        {
            println!("XMAS ur {} ({},{})", i, col, row);
            found += 1;
        }
        if row < lines_count - 3
            && col >= 3
            && chars.get(i - 1 + 1 * line_len).unwrap() == &'M'
            && chars.get(i - 2 + 2 * line_len).unwrap() == &'A'
            && chars.get(i - 3 + 3 * line_len).unwrap() == &'S'
        {
            println!("XMAS dl {} ({},{})", i, col, row);
            found += 1;
        }
        if row < lines_count - 3
            && col < line_len - 3
            && chars.get(i + 1 + 1 * line_len).unwrap() == &'M'
            && chars.get(i + 2 + 2 * line_len).unwrap() == &'A'
            && chars.get(i + 3 + 3 * line_len).unwrap() == &'S'
        {
            println!("XMAS dr {} ({},{})", i, col, row);
            found += 1;
        }
    }

    // TODO:
    //  * count "XMAS" + "SAMX"
    //  * count "X" "S"
    //          "M" "A"
    //          "A" "M"
    //          "S" "X"
    //  * count diagonals: both + both directions
    //
    // Strategies:
    //  * transpose char matrix?
    //  * find "X" + check all directions from there?!
    //
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
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
