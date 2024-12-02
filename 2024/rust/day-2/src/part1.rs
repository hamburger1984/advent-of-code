use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let safe_reports = input
        .lines()
        .map(|l| {
            let levels = l
                .split(' ')
                .map(|n| n.parse::<u16>().unwrap())
                .collect_vec();

            return is_safe_up(&levels) || is_save_down(&levels);
        })
        .filter(|safe| *safe)
        .count();

    return Ok(safe_reports.to_string());
}

fn is_save_down(levels: &Vec<u16>) -> bool {
    let mut last = levels.first().unwrap();
    for level in levels[1..].iter() {
        if level >= last || level + 3 < *last {
            return false;
        }
        last = level;
    }

    return true;
}

fn is_safe_up(levels: &Vec<u16>) -> bool {
    let mut last = levels.first().unwrap();
    for level in levels[1..].iter() {
        if level <= last || *level > last + 3 {
            return false;
        }
        last = level;
    }

    return true;
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
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
