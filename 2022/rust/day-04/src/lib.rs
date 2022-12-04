pub fn process_part1(input: &str) -> String {
    let sum = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let (b1, b2) = b.split_once('-').unwrap();

            let ia1 = a1.parse::<u64>().unwrap();
            let ia2 = a2.parse::<u64>().unwrap();
            let ib1 = b1.parse::<u64>().unwrap();
            let ib2 = b2.parse::<u64>().unwrap();

            //dbg!(line, ia1, ia2, ib1, ib2);

            if ia1 <= ib1 && ib2 <= ia2 {
                return 1;
            }
            if ib1 <= ia1 && ia2 <= ib2 {
                return 1;
            }

            return 0;
        })
        .sum::<u64>();

    return sum.to_string();
}

pub fn process_part2(input:&str) -> String {
    let sum = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let (b1, b2) = b.split_once('-').unwrap();

            let ia1 = a1.parse::<u64>().unwrap();
            let ia2 = a2.parse::<u64>().unwrap();
            let ib1 = b1.parse::<u64>().unwrap();
            let ib2 = b2.parse::<u64>().unwrap();


            if ia1 <= ib1 && ib1 <= ia2 {
                return 1;
            }
            if ia1 <= ib2 && ib2 <= ia2 {
                return 1;
            }

            if ib1 <= ia1 && ia1 <= ib2 {
                return 1;
            }
            if ib1 <= ia2 && ia2 <= ib2 {
                return 1;
            }

            return 0;
        })
        .sum::<u64>();

    return sum.to_string();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = process_part1(TESTINPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part2() {
        let result = process_part2(TESTINPUT);
        assert_eq!(result, "4");
    }

    const TESTINPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
}
