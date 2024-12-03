use regex::{Captures, Regex};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let disabled = Regex::new(r"don't\(\)(.*?)(do\(\)|$)").unwrap();
    let mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let no_breaks = input.replace("\n", " ");

    let colored = disabled.replace_all(no_breaks.as_str(), |c: &Captures| {
        format!("\x1b[33mdon't()\x1b[31m{}\x1b[33m{}\x1b[32m", &c[1], &c[2])
    });

    println!("\x1b[32m{}\x1b[0m", colored);

    let enabled_only = disabled.replace_all(no_breaks.as_str(), "");

    let sum = mul
        .captures_iter(enabled_only.as_ref())
        .map(|c| {
            let m = c.get(0).unwrap();
            println!("{} ({},{})", m.as_str(), m.start(), m.end());
            c.extract()
        })
        .map(|(_, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
        .sum::<u32>();

    return Ok(sum.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
