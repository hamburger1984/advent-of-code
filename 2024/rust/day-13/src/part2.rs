use std::cmp;

use itertools::Itertools;
use regex::Regex;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // A: 3 Tokens
    // B: 1 Token
    let mut total_tokens = 0;
    for machines in input.lines().filter(|l| l.len() > 0).chunks(3).into_iter() {
        let (a, b, prize) = machines.collect_tuple().unwrap();
        let ((ax, ay), (bx, by), (px, py)) = (parse_xy(a), parse_xy(b), parse_xy(prize));
        let (px, py) = (px + 10000000000000, py + 10000000000000);
        println!("A {},{} B {},{} P {},{}", ax, ay, bx, by, px, py);

        // px = ax * p1 + bx * p2
        // px - bx*p2 = ax*p1 | :ax
        // (px - bx*p2)/ax = p1
        //
        // py = ay * p1 + by * p2 | replace p1
        // px = ay * (px-bx*p2)/ax + by * p2 | - by*p2
        // px - by*p2 = ay * (px-bx*p2)/ax | * ax
        // ax * (px - by*p2) = ay * (px-bx*p2)
        // ax*py - ax*by*p2 = ay*px - ay*bx*p2 | + ax*by*p2 | - ay*px
        // ax*py - ay*px = ax*by*p2 - ay*bx*p2
        let left_side = ax * py - ay * px;
        let r_fac_1 = ax * by;
        let r_fac_2 = ay * bx;
        let right_side = r_fac_1 - r_fac_2;

        if left_side % right_side == 0 {
            let p2 = left_side / right_side;
            let p1 = (px - p2 * bx) / ax;
            let tokens = p1 * 3 + p2;
            println!(
                "YEAH {},{} total {} -> {}",
                p1,
                p2,
                total_tokens,
                total_tokens + tokens
            );
            total_tokens += tokens;
        }
    }
    Ok(total_tokens.to_string())
}

fn parse_xy(line: &str) -> (i64, i64) {
    let xy = Regex::new(r"X.(\d+),\sY.(\d+)").unwrap();
    let cap = xy.captures(line).unwrap();

    (
        cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
        cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
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
