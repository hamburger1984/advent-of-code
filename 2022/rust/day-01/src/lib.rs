use std::{cmp, process::exit};

pub fn process_part1(input: &str) -> String {
    let mut fattest_elf = 0;
    let mut current_elf = 0;

    for line in input.lines() {
        if line.is_empty() {
            current_elf = 0;
            continue;
        }

        if let Ok(cal) = line.parse::<u32>() {
            current_elf += cal;
            fattest_elf = cmp::max(fattest_elf, current_elf);
        } else {
            eprint!("Could not parse line '{}'", line);
            exit(1);
        }
    }

    return fattest_elf.to_string();
}

pub fn process_part2(input: &str) -> String {
    let mut all_elves = Vec::new();
    all_elves.push(0);

    for line in input.lines() {
        if line.is_empty() {
            all_elves.insert(0, 0);
            continue;
        }

        if let Ok(cal) = line.parse::<u32>() {
            all_elves[0] += cal;
        } else {
            eprint!("Could not parse line '{}'", line);
            exit(1);
        }
    }

    all_elves.sort();
    all_elves.reverse();
    all_elves.truncate(3);

    let mut elves_sum = 0;
    for top_elf in all_elves {
        print!("Top Elf: {}\n", top_elf);
        elves_sum += top_elf;
    }

    return elves_sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTINPUT: &str= "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_part1() {
        let result = process_part1(TESTINPUT);

        assert_eq!(result, "24000");
    }

    #[test]
    fn test_part2() {
        let result = process_part2(TESTINPUT);

        assert_eq!(result, "45000");
    }
}
