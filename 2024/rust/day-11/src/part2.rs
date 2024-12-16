use cached::proc_macro::cached;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut total_stones = 0;
    for s in stones {
        total_stones += total(s, 75);
    }

    Ok(total_stones.to_string())
}

#[cached]
fn total(stone: u64, remaining: u8) -> u64 {
    if remaining == 0 {
        return 1u64;
    }

    if stone == 0 {
        return total(1, remaining - 1);
    } else {
        let s_str = stone.to_string();

        if s_str.len() % 2 == 0 {
            let left = s_str[..s_str.len() / 2].parse::<u64>().unwrap();
            let right = s_str[s_str.len() / 2..].parse::<u64>().unwrap();
            return total(left, remaining - 1) + total(right, remaining - 1);
        } else {
            return total(stone * 2024, remaining - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
