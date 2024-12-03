use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (mut good, mut bad, mut total) = (0, 0, 0);

    let safe_reports = input
        .lines()
        .map(|l| {
            total += 1;
            let levels = l
                .split(' ')
                .map(|n| n.parse::<i16>().unwrap())
                .collect_vec();

            print!("{}", l);

            for skip_idx in 0..levels.len() {
                print!("\nskip {}", skip_idx);

                let levels_with_skipped = levels[..skip_idx]
                    .iter()
                    .chain(levels[(skip_idx + 1)..].iter())
                    .map(|p| *p)
                    .collect::<Vec<i16>>();

                let pairs = levels_with_skipped
                    .iter()
                    .zip(levels_with_skipped.iter().skip(1));

                let diffs = pairs
                    .map(|p| {
                        print!(" ({}, {}): {}", p.0, p.1, p.1 - p.0);
                        p.1 - p.0
                    })
                    .collect::<Vec<i16>>();

                if diffs.iter().all(|d| 0 < *d && *d <= 3)
                    || diffs.iter().all(|d| 0 > *d && *d >= -3)
                {
                    good += 1;
                    println!(" >> good");
                    return true;
                }
            }

            //println!("{} >> bad", l);
            println!(" >> bad");
            bad += 1;
            return false;
        })
        .filter(|safe| *safe)
        .count();

    println!("good {} and bad {} of {}", good, bad, total);

    return Ok(safe_reports.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
