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

        let mut min_tokens = i64::MAX;

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
            println!(" YEAH {},{} ", p1, p2);
            let tokens = p1 * 3 + p2;
            min_tokens = tokens;
        }

        //let max = 1 + cmp::min(px / bx, py / by);

        //for i in 0..max {
        //    let p2 = max - i;
        //    //print!("{} ", p2);
        //    if left_side == right_side * p2 {
        //        let p1 = (px - p2 * bx) / ax;
        //        println!(" YEAH {},{} ", p1, p2);

        //        let tokens = p1 * 3 + p2;
        //        if tokens < min_tokens {
        //            min_tokens = tokens;
        //        }
        //    }
        //}

        //let mut rx = px % ax;
        //let mut ry = py % ay;
        //while rx <= px && ry <= py {
        //    println!(
        //        " .. {},{} % b {},{} = {},{}",
        //        rx,
        //        ry,
        //        bx,
        //        by,
        //        rx % bx,
        //        ry % by
        //    );
        //    if rx % bx == 0 && ry % by == 0 && rx / bx == ry / by {
        //        let a = (px - px % ax) / ax;
        //        println!(
        //            "a {} r/b {} a {},{} b {},{} p {},{} r {},{}",
        //            a,
        //            rx / bx,
        //            ax,
        //            ay,
        //            bx,
        //            by,
        //            px,
        //            py,
        //            rx,
        //            ry
        //        );
        //        let tokens = a * 3 + (rx / bx);
        //        if tokens < min_tokens {
        //            println!("min {} -> {}", min_tokens, tokens);
        //            min_tokens = tokens;
        //        }
        //    }
        //    rx += ax;
        //    ry += ay;
        //}

        if min_tokens < i64::MAX {
            println!("total {} -> {}", total_tokens, total_tokens + min_tokens);
            total_tokens += min_tokens;
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
