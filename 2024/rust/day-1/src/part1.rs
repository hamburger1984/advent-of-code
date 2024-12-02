use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|l| {
            l.split_once("   ")
                .map(|pair| {
                    (
                        pair.0.parse::<i32>().unwrap(),
                        pair.1.parse::<i32>().unwrap(),
                    )
                })
                .unwrap()
        })
        .unzip();

    left.sort();
    right.sort();

    let sum_of_diffs = left
        .into_iter()
        .zip(right.into_iter())
        .map(|p| if p.0 == p.1 { 0 } else { i32::abs(p.0 - p.1) })
        .sum::<i32>();

    return Ok(sum_of_diffs.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
