use std::collections::HashSet;

pub fn process_part1(input: &str) -> String {
    let upper_a = b'A';
    let lower_a = b'a';

    let error_sum = input
        .lines()
        .map(|line| {
            let bytes: Vec<u8> = line.bytes().collect();
            let first_half = bytes.iter().take(bytes.len() / 2).collect::<HashSet<_>>();
            let second_half = bytes.iter().skip(bytes.len() / 2).collect::<HashSet<_>>();

            let mut error: u64 = 0;
            for common_item in first_half.intersection(&second_half) {
                let char_error = if **common_item > lower_a {
                    **common_item - lower_a + 1
                } else {
                    **common_item - upper_a + 27
                };

                error = error + char_error as u64;
            }

            return error;
        })
        .sum::<u64>();

    return error_sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_part1() {
        let result = process_part1(TESTINPUT);
        assert_eq!(result, "157");
    }
}
