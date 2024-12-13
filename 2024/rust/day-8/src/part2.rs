use std::collections::HashSet;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let width = input.split_once('\n').unwrap().0.len() as i32;
    let height = input.lines().count() as i32;

    let mut antennas: Vec<(char, i32, i32)> = Vec::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                antennas.push((c, x as i32, y as i32));
            }
        }
    }

    let mut antis: HashSet<(i32, i32)> = HashSet::new();

    for a in 0..antennas.len() {
        for b in (a + 1)..antennas.len() {
            println!("{}x{}", a, b);
            let (a_c, a_x, a_y) = antennas[a];
            let (b_c, b_x, b_y) = antennas[b];
            if a_c != b_c {
                continue;
            }
            let (d_x, d_y) = (b_x - a_x, b_y - a_y);
            println!(
                "{} ({},{}) -- {},{} -> ({},{})",
                a_c, a_x, a_y, d_x, d_y, b_x, b_y
            );

            antis.insert((a_x, a_y));
            let (mut ad_x, mut ad_y) = (a_x - d_x, a_y - d_y);
            while ad_x >= 0 && ad_x < width && ad_y >= 0 && ad_y < height {
                antis.insert((ad_x, ad_y));
                (ad_x, ad_y) = (ad_x - d_x, ad_y - d_y);
            }

            antis.insert((b_x, b_y));
            let (mut bd_x, mut bd_y) = (b_x + d_x, b_y + d_y);
            while bd_x >= 0 && bd_x < width && bd_y >= 0 && bd_y < height {
                antis.insert((bd_x, bd_y));
                (bd_x, bd_y) = (bd_x + d_x, bd_y + d_y);
            }
        }
    }

    Ok(antis.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("34", process(input)?);
        Ok(())
    }
}
