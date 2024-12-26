use itertools::Itertools;
use regex::Regex;
use std::cmp;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // A: 3 Tokens
    // B: 1 Token
    let mut total_tokens = 0;
    for machines in input.lines().filter(|l| l.len() > 0).chunks(3).into_iter() {
        let (a, b, prize) = machines.collect_tuple().unwrap();
        let ((ax, ay), (bx, by), (px, py)) = (parse_xy(a), parse_xy(b), parse_xy(prize));
        println!("{}, {} {}", ax, px, px / ax);

        let mut min_tokens = u32::MAX;

        for i in 0..cmp::min(100, 1 + cmp::min(px / ax, py / ay)) {
            let rx = px - i * ax;
            let ry = py - i * ay;

            if rx % bx == 0 && ry % by == 0 && rx / bx == ry / by {
                println!(
                    "i {} r/b {} a {},{} b {},{} p {},{} r {},{}",
                    i,
                    rx / bx,
                    ax,
                    ay,
                    bx,
                    by,
                    px,
                    py,
                    rx,
                    ry
                );
                let tokens = i * 3 + (rx / bx);
                if tokens < min_tokens {
                    println!("min {} -> {}", min_tokens, tokens);
                    min_tokens = tokens;
                }
            }
        }
        if min_tokens < u32::MAX {
            println!("total {} -> {}", total_tokens, total_tokens + min_tokens);
            total_tokens += min_tokens;
        }
    }
    Ok(total_tokens.to_string())
}

fn parse_xy(line: &str) -> (u32, u32) {
    let xy = Regex::new(r"X.(\d+),\sY.(\d+)").unwrap();
    let cap = xy.captures(line).unwrap();

    (
        cap.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        cap.get(2).unwrap().as_str().parse::<u32>().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
