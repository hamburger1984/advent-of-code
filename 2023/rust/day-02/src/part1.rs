use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let possible_games = input.lines().filter_map(game_id_if_possible).sum::<u32>();
    Ok(possible_games.to_string())
}

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn game_id_if_possible(line: &str) -> Option<u32> {
    let colon = line.find(':').expect("line should have a colon");
    let game_id = line["Game ".len()..colon].parse::<u32>().expect("game id should be numeric");

    if line[colon + 1..].split(';').any(is_impossible) {
        return None;
    }

    return Some(game_id);
}

fn is_impossible(cubes_drawn: &str) -> bool {
    return cubes_drawn.trim().split(',').any(|number_and_cube| {
        let (number, color) = number_and_cube.trim().split_once(' ').expect("each drawing should be '<number> <cube color>'");
        //println!("{} {}", number, color);
        let parsed = number.parse::<u32>().expect("number of cubes expected");
        if color == "red" && parsed > RED { true } else if color == "green" && parsed > GREEN { true } else if color == "blue" && parsed > BLUE { true } else { false }
    });
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
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
