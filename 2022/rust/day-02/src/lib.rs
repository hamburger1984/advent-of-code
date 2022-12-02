use std::{collections::HashMap, process::exit};

pub fn process_part1(input: &str) -> String {
/*
A/X = Rock     + 1
B/Y = Paper    + 2
C/Z = Scissors + 3

Win = 6
Draw = 3
Loose = 0
 */

    let scores:HashMap<&str, u64> = HashMap::from([
        ("A X", 3 + 1),
        ("A Y", 6 + 2),
        ("A Z", 0 + 3),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 6 + 1),
        ("C Y", 0 + 2),
        ("C Z", 3 + 3),
    ]);

    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if let Some(score) = scores.get(line) {
            sum += score;
        } else {
            exit(1);
        }
    }

    return sum.to_string();
}

pub fn process_part2(input: &str) -> String {
/*
A = Rock     + 1
B = Paper    + 2
C = Scissors + 3

X = loose
Y = draw
Z = win

Win = 6
Draw = 3
Loose = 0
 */
    let scores:HashMap<&str, u64> = HashMap::from([
        ("A X", 3 + 0), // rock and I want to lose -> scissors(3) + 0
        ("A Y", 1 + 3),
        ("A Z", 2 + 6),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 2 + 0),
        ("C Y", 3 + 3),
        ("C Z", 1 + 6),
    ]);

    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if let Some(score) = scores.get(line) {
            //print!("{} -> {}\n", line, score);
            sum += score;
        } else {
            exit(1);
        }
    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
A Y
B X
C Z
";

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
