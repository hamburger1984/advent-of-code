use crate::custom_error::AocError;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (left, right): (Vec<i32>, Vec<i32>) = input
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

    let right_counts = right.into_iter().counts();

    let num_times_occurences = left
        .into_iter()
        .map(|l| l as usize * right_counts.get(&l).unwrap_or(&(0usize)))
        .sum::<usize>();

    return Ok(num_times_occurences.to_string());
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
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
