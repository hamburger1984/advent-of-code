use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // blink 25 times
    let mut stones = input
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    for _blink in 0..25 {
        stones = blink(stones);
    }

    Ok(stones.len().to_string())
}

fn blink(stones: Vec<u128>) -> Vec<u128> {
    let mut result = Vec::new();

    // 0 becomes 1
    // even number of digits (abcd) becomes two stones (ab) (cd)
    // else stone with x becomes stone with x*2024
    for s in stones {
        if s == 0 {
            result.push(1);
            continue;
        }

        let str = s.to_string();
        if str.len() % 2 == 0 {
            result.push(str[..str.len() / 2].parse::<u128>().unwrap());
            result.push(str[(str.len() / 2)..].parse::<u128>().unwrap());
        } else {
            result.push(s * 2024)
        }
    }
    return result;
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
