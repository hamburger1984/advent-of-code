use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut lines = input.lines();

    let rules = lines
        .by_ref()
        .take_while(|l| l.len() > 1)
        .map(|r| {
            let (a, b) = r.split_once('|').unwrap();
            return (a.parse::<u16>().unwrap(), b.parse::<u16>().unwrap());
        })
        .collect::<Vec<(u16, u16)>>();

    let updates = lines
        .by_ref()
        .skip_while(|l| l.len() == 0)
        .take_while(|l| l.len() > 0)
        .map(|l| {
            l.split(',')
                .map(|p| p.parse::<u16>().unwrap())
                .collect::<Vec<u16>>()
        })
        .collect::<Vec<Vec<u16>>>();

    let bad_updates = updates
        .iter()
        .filter(|update| {
            for i in 1..update.len() {
                if rules
                    .iter()
                    .any(|(a, b)| *a == update[i] && update[..i].iter().any(|seen| seen == b))
                {
                    return true;
                }
            }
            return false;
        })
        .collect::<Vec<&Vec<u16>>>();

    let mut sum = 0;
    for update in bad_updates {
        let mut fixed_update = update.iter().collect::<Vec<&u16>>();
        let mut i = 0;

        while i < fixed_update.len() {
            if let Some((broken_a, broken_b)) = rules.iter().find(|(a, b)| {
                b == fixed_update[i] && fixed_update[i..].iter().any(|remaining| *remaining == a)
            }) {
                let bad_index = fixed_update.iter().position(|x| *x == broken_a).unwrap();
                fixed_update.remove(bad_index);
                fixed_update.insert(i, broken_a);
                println!(
                    "moved {} from {} to {} rule was ({},{})",
                    broken_a, bad_index, i, broken_a, broken_b
                );
            } else {
                i += 1;
            }
        }

        sum += fixed_update[fixed_update.len() / 2];
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
