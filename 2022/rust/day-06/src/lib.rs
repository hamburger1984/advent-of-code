use std::collections::HashSet;

pub fn process_part1(input: &str) -> String {
    let mut buffer = vec![' ', ' ', ' ', ' '];

    for (i, c) in input.char_indices() {
        buffer[i % 4] = c;
        if i >= 3 {
            let unique = buffer.iter().collect::<HashSet<_>>();

            if unique.len() == 4 {
                return (i + 1).to_string();
            }
        }
    }

    return "NOPE".to_string();
}

pub fn process_part2(input: &str) -> String {
    let mut buffer = vec![' ', ' ', ' ', ' ', ' ',' ', ' ', ' ', ' ', ' ',' ', ' ', ' ', ' ' ];

    for (i, c) in input.char_indices() {
        buffer[i % 14] = c;
        if i >= 13 {
            let unique = buffer.iter().collect::<HashSet<_>>();

            if unique.len() == 14 {
                return (i + 1).to_string();
            }
        }
    }

    return "NOPE".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expectations: [&str; 5] = ["7", "5", "6", "10", "11"];
        for i in 0..5 {
            let result = process_part1(TESTINPUTS[i]);
            assert_eq!(result, expectations[i]);
        }
    }

    #[test]
    fn test_part2() {
        let expectations: [&str; 5] = ["19", "23", "23", "29", "26"];
        for i in 0..5 {
            let result = process_part2(TESTINPUTS[i]);
            assert_eq!(result, expectations[i]);
        }
    }

    const TESTINPUTS: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
}
