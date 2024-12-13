use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut total_calibration = 0;

    for line in input.lines() {
        // Parse the input line
        if let Some((target, numbers)) = line.split_once(": ") {
            let target_value: u64 = target.parse().unwrap();
            let nums: Vec<u64> = numbers
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            if is_valid(target_value, nums) {
                total_calibration += target_value;
            }
        }
    }

    Ok(total_calibration.to_string())
}

fn is_valid(target: u64, nums: Vec<u64>) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }
    let mut add = vec![nums[0] + nums[1]];
    if nums.len() > 2 {
        let mut rest = nums[2..].into_iter().map(|n| *n).collect::<Vec<u64>>();
        add.append(&mut rest);
    }
    if is_valid(target, add) {
        return true;
    }

    let mut multi = vec![nums[0] * nums[1]];
    if nums.len() > 2 {
        let mut rest = nums[2..].into_iter().map(|n| *n).collect::<Vec<u64>>();
        multi.append(&mut rest);
    }
    if is_valid(target, multi) {
        return true;
    }
    let mut conc = vec![format!("{}{}", nums[0], nums[1]).parse().unwrap()];
    if nums.len() > 2 {
        let mut rest = nums[2..].into_iter().map(|n| *n).collect::<Vec<u64>>();
        conc.append(&mut rest);
    }
    if is_valid(target, conc) {
        return true;
    }
    return false;
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
        assert_eq!("11387", process(input)?);
        Ok(())
    }
}
