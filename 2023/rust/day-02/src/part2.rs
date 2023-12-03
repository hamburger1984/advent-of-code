use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    return Ok(input.lines().map(game_value).sum::<u32>().to_string());
}

pub fn game_value(line: &str) -> u32 {
    let colon = line.find(':').expect("line should have a colon");
    let mut max_r = 0;
    let mut max_g = 0;
    let mut max_b = 0;
    for (r, g, b) in line[colon + 1..].split(';').map(num_cubes) {
        if r > max_r { max_r = r; }
        if g > max_g { max_g = g; }
        if b > max_b { max_b = b; }
    }
    return max_r * max_g * max_b;
}

fn num_cubes(cubes_drawn: &str) -> (u32, u32, u32) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for number_and_cube in cubes_drawn.trim().split(',') {
        let (number, color) = number_and_cube.trim().split_once(' ').expect("each drawing should be '<number> <cube color>'");
        let parsed = number.parse::<u32>().expect("number of cubes expected");
        if color == "red" { r = parsed; }
        if color == "green" { g = parsed; }
        if color == "blue" { b = parsed; }
    }
    (r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }

    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_min_cubes(#[case] line: &str, #[case] expected: u32) -> miette::Result<()> {
        assert_eq!(expected, game_value(line));
        Ok(())
    }
}