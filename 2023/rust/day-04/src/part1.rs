use itertools::Itertools;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    return Ok(input.lines().filter_map(card_worth).sum::<u32>().to_string());
}

fn card_worth(card_line: &str) -> Option<u32> {
    let (_, numbers) = card_line.split_once(":").unwrap();

    let (left, right) = numbers.split_once("|").unwrap();

    let right_split = right.split_whitespace().collect_vec();

    let common = left.split_whitespace().filter_map(|e| { if !right_split.contains(&e) { None } else { Some(e) } }).count();

    if common == 0 { Some(0) } else {
        let base: u32 = 2;
        Some(base.pow((common - 1) as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
