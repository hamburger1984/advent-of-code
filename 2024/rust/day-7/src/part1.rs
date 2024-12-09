use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut valid_lines_sum = 0;
    for l in input.lines() {
        if let Some((result, numbers)) = l.split_once(": ") {
            let expected = result.parse::<u128>().unwrap();
            let numbers = numbers
                .split(' ')
                .map(|n| n.parse::<u128>().unwrap())
                .collect::<Vec<u128>>();

            for i in 0..u32::pow(2, (numbers.len() as u32) - 1) {
                //print!("{} ", numbers[0]);

                let mut res = numbers[0];
                let mut mask = 0b1;
                for n in numbers[1..].iter() {
                    if i & mask != 0 {
                        res = res * n;
                        //print!("* {} ", n);
                    } else {
                        res = res + n;
                        //print!("+ {} ", n);
                    }
                    mask = mask << 1;
                }
                //println!(" ?= {} {} {:#b}", expected, i, mask);

                if res == expected {
                    valid_lines_sum += expected;
                    break;
                }
            }
        }
    }

    Ok(valid_lines_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
