use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut total = 0;
    for line in input.lines() {
        let mut first_digit = '-';
        let mut last_digit = '-';

        for c in line.chars() {
            if c.is_digit(10) && c != '0' {
                if first_digit == '-' {
                    first_digit = c;
                }
                last_digit = c;
            }
        }

        total += first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();
    }

    return Result::Ok(total.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
